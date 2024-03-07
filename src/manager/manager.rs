use std::{
    error::Error,
    fs,
    net::SocketAddr,
    path::PathBuf,
    sync::{self, Arc},
};

use tokio_util::sync::CancellationToken;

use crate::{
    database::{self, *},
    service::{self, Service},
};

#[derive(Debug, serde::Deserialize)]
pub struct ManagerRawOptions {
    log_level: String,
    log_file: Option<String>,
    database_url: Option<String>,
    secret: String,
    listen: SocketAddr,
    local_listen_port: Option<u16>,
    data_dir: String,
    temp_dir: String,
}

pub struct ManagerOptions {
    pub log_level: String,
    pub log_file: crate::log::LogOutput,
    pub database_url: Option<String>,
    pub secret: String,
    pub listen: SocketAddr,
    pub local_listen_port: Option<u16>,
    pub data_dir: PathBuf,
    pub temp_dir: PathBuf,
}

impl TryFrom<ManagerRawOptions> for ManagerOptions {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(options: ManagerRawOptions) -> Result<Self, Self::Error> {
        let log_file = match options.log_file {
            Some(f) => match f.as_str() {
                "stdout" | "" => crate::log::LogOutput::stdout(),
                "stderr" => crate::log::LogOutput::stderr(),
                "off" => crate::log::LogOutput::nop(),
                _ => crate::log::LogOutput::file(&f).map_err(|e| e.to_string())?,
            },
            None => crate::log::LogOutput::stdout(),
        };
        Ok(ManagerOptions {
            log_level: options.log_level,
            log_file,
            database_url: options.database_url,
            secret: options.secret,
            listen: options.listen,
            local_listen_port: options.local_listen_port,
            data_dir: options.data_dir.into(),
            temp_dir: options.temp_dir.into(),
        })
    }
}

pub struct Manager {
    database_url: Option<String>,
    database: sync::RwLock<Option<Database>>,
    service: sync::RwLock<Option<Service>>,
    http_server: sync::Mutex<Option<super::HTTPServer>>,
    data_dir: PathBuf,
    temp_dir: PathBuf,
    exit_token: CancellationToken,
}

impl Manager {
    pub async fn prepare(
        options: ManagerOptions,
    ) -> Result<Arc<Self>, Box<dyn Error + Send + Sync>> {
        // Set Logger
        let logger = crate::log::Logger::new(&options.log_level, options.log_file.to_box_writer())
            .map_err(|e| Into::<Box<dyn Error + Send + Sync>>::into(e.to_string()))?;
        logger
            .set_global()
            .map_err(|e| Into::<Box<dyn Error + Send + Sync>>::into(e.to_string()))?;
        let s = Arc::new(Self {
            database_url: options.database_url,
            database: sync::RwLock::new(None),
            service: sync::RwLock::new(None),
            http_server: sync::Mutex::new(None),
            data_dir: options.data_dir,
            temp_dir: options.temp_dir,
            exit_token: CancellationToken::new(),
        });
        // Set Service
        let service = service::Service::new(s.clone());
        *s.service.write().unwrap() = Some(service);
        //
        // Set HTTP Server
        let http_server = super::HTTPServer::new(
            s.clone(),
            options.listen,
            options.secret,
            options.local_listen_port,
        );
        *s.http_server.lock().unwrap() = Some(http_server);
        //
        Ok(s)
    }

    pub async fn run(
        &self,
        cancel_token: CancellationToken,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        log::info!("Manager is running");
        if let Err(e) = fs::create_dir_all(&self.data_dir) {
            return Err(format!("create data dir failed: {}", e).into());
        }
        if let Err(e) = fs::create_dir_all(&self.temp_dir) {
            return Err(format!("create temp dir failed: {}", e).into());
        }
        // Set Database
        log::info!("Database is connecting...");
        let database_url = match &self.database_url {
            Some(url) => url.clone(),
            None => format!(
                "sqlite://{}?mode=rwc",
                self.data_dir.join("data.db").to_string_lossy()
            ),
        };
        let db = database::Database::new(&database_url).await.map_err(|e| {
            Into::<Box<dyn Error + Send + Sync>>::into(format!("Database connect failed: {}", e))
        })?;
        *self.database.write().unwrap() = Some(db);
        log::info!("Database is connected");
        //
        let http_server = self.http_server.lock().unwrap().take().unwrap();
        let service = self.get_service();
        log::info!("Service is starting...");
        service.start().await.map_err(|e| {
            Into::<Box<dyn Error + Send + Sync>>::into(format!("Service Start Error: {}", e))
        })?;
        log::info!("Service is started");
        log::info!("HTTP Server is running on {}", &http_server.listen);
        let exit_token = self.exit_token.clone();
        tokio::select! {
            _ = cancel_token.cancelled() => {}
            _ = exit_token.cancelled() => {
                log::warn!("request to exit...");
                cancel_token.cancel();
            }
            res = http_server.run() => {
                res?
            }
        }
        log::warn!("HTTP Server is stopped");
        if let Some(db) = self.database.read().unwrap().clone().take() {
            log::info!("Close Database Connection");
            let _ = db.close().await;
        }
        let _ = service.close().await;
        log::info!("Service is stopped");
        log::info!("Manager is stopped");
        Ok(())
    }

    pub(crate) fn get_database(&self) -> Database {
        self.database.read().unwrap().clone().unwrap()
    }

    pub(crate) fn get_service(&self) -> Service {
        self.service.read().unwrap().clone().unwrap()
    }

    pub(crate) fn get_data_dir_path(&self) -> &PathBuf {
        &self.data_dir
    }

    pub(crate) fn get_temp_dir_path(&self) -> &PathBuf {
        &self.temp_dir
    }

    pub(crate) fn request_exit(&self) {
        self.exit_token.cancel();
    }
}

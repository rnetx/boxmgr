use std::{
    error::Error,
    fs,
    net::SocketAddr,
    process::Stdio,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::{mpsc, Mutex, Notify},
};
use tokio_util::sync::CancellationToken;

use crate::{common, database, manager::Manager};

pub(crate) struct Status {
    pub(crate) is_running: AtomicBool,
    pub(crate) running_config: RwLock<String>,
    pub(crate) core_version: RwLock<String>,
    pub(crate) memory_usage: AtomicU64,       // B
    pub(crate) connection_count: AtomicUsize, // count
    pub(crate) upload_traffic: AtomicU64,     // B
    pub(crate) download_traffic: AtomicU64,   // B
    pub(crate) upload_speed: AtomicU64,       // B/s
    pub(crate) download_speed: AtomicU64,     // B/s
}

impl Status {
    pub(crate) fn clean_data(&self) {
        self.memory_usage.store(0, Ordering::Relaxed);
        self.connection_count.store(0, Ordering::Relaxed);
        self.upload_traffic.store(0, Ordering::Relaxed);
        self.download_traffic.store(0, Ordering::Relaxed);
        self.upload_speed.store(0, Ordering::Relaxed);
        self.download_speed.store(0, Ordering::Relaxed);
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            running_config: RwLock::new(String::new()),
            core_version: RwLock::new(String::new()),
            memory_usage: AtomicU64::new(0),
            connection_count: AtomicUsize::new(0),
            upload_traffic: AtomicU64::new(0),
            download_traffic: AtomicU64::new(0),
            upload_speed: AtomicU64::new(0),
            download_speed: AtomicU64::new(0),
        }
    }
}

struct ServiceInner {
    token: CancellationToken,
    receiver: mpsc::Receiver<()>,
    config: database::Config,
    status: Arc<super::State<Status>>,
}

impl ServiceInner {
    async fn new(
        manager: Arc<Manager>,
        core_path: String,
        mut config: database::Config,
        log_queue: Arc<super::LogQueue<String>>,
        status: Arc<super::State<Status>>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        *status.running_config.write().unwrap() = config.tag.clone();
        status.notify();
        let script_handler = super::ScriptHandler::new(manager).await?;
        // Check Config
        let (listen, secret) = Self::check_config(&mut config.config);
        let listen = SocketAddr::from_str(&listen)?;

        // Set Permission
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let f = fs::File::open(&core_path)?;
            let metadata = f.metadata()?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
        }

        // Get Core Info
        let version_output = Command::new(&core_path)
            .args(["version"])
            .output()
            .await
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())?;
        for line in version_output.split('\n').collect::<Vec<&str>>() {
            let line = line.trim();
            if line.starts_with("sing-box version") {
                let version = line.trim_start_matches("sing-box version").trim();
                *status.core_version.write().unwrap() = version.to_owned();
                status.notify();
                log::debug!("service: core version: {}", version);
            } else if line.starts_with("Tags:") {
                let tag_str = line.trim_start_matches("Tags:").trim();
                let tags = tag_str
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<String>>();
                log::debug!("service: core tags: {:?}", tags);
            }
        }
        //
        let config_content = config.config.to_string();
        let mut cmd = Command::new(core_path);
        cmd.args(["run", "--config", "stdin", "--disable-color"]);
        cmd.kill_on_drop(true);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        #[cfg(windows)]
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW

        script_handler.run_before_start_script().await;

        let mut child = cmd.spawn().map_err(|e| {
            log::error!("service: start service failed: {}", &e);
            Into::<Box<dyn Error + Send + Sync>>::into(format!(
                "service: start service failed: {}",
                e
            ))
        })?;
        match &mut child.stdin {
            Some(stdin) => {
                stdin
                    .write_all(config_content.as_bytes())
                    .await
                    .map_err(|e| {
                        log::error!("service: write config to stdin failed: {}", &e);
                        format!("service: write config to stdin failed: {}", e)
                    })?;
            }
            None => {
                log::error!("service: stdin is not piped");
                return Err("service: stdin is not piped".into());
            }
        }
        let (sender, receiver) = mpsc::channel(1);
        let started_notify_handle = Arc::new(Notify::new());
        let token = CancellationToken::new();
        let token_handle = token.clone();
        let status_handle = status.clone();
        let status_clash_api_handle = status.clone();
        let token_clash_api_handle = token.clone();
        let sender_clash_api_handle = sender.clone();
        let started_notify_clash_api_handle = started_notify_handle.clone();
        tokio::spawn(async move {
            Self::clash_api_handle(
                listen,
                secret,
                started_notify_clash_api_handle,
                status_clash_api_handle,
                token_clash_api_handle,
                sender_clash_api_handle,
            )
            .await
        });
        tokio::spawn(async move {
            Self::child_handle(
                token_handle,
                sender,
                child,
                script_handler,
                log_queue,
                started_notify_handle,
                status_handle,
            )
            .await
        });
        Ok(Self {
            token,
            receiver,
            config,
            status,
        })
    }

    fn check_config(config: &mut serde_json::Value) -> (String, Option<String>) {
        let map = match config {
            serde_json::Value::Object(m) => m,
            _ => unreachable!(),
        };
        if let Some(serde_json::Value::Object(log)) = map.get_mut("log") {
            log.remove("disabled");
            log.insert("output".into(), "stdout".into());
            log.insert("timestamp".into(), false.into());
        }
        let init_clash_api_map =
            |clash_api_map: &mut serde_json::Map<String, serde_json::Value>,
             init_secret: bool|
             -> (String, Option<String>) {
                const DEFAULT_CLASH_API_LISTEN: &str = "127.0.0.1:9090";

                let listen = if let Some(serde_json::Value::String(external_controller)) =
                    clash_api_map.get("external_controller")
                {
                    external_controller.clone()
                } else {
                    clash_api_map.insert(
                        "external_controller".into(),
                        DEFAULT_CLASH_API_LISTEN.into(),
                    );
                    DEFAULT_CLASH_API_LISTEN.to_string()
                };
                let secret =
                    if let Some(serde_json::Value::String(secret)) = clash_api_map.get("secret") {
                        Some(secret.clone())
                    } else if init_secret {
                        let secret = common::random_uuid().replace("-", "");
                        clash_api_map.insert("secret".into(), secret.clone().into());
                        Some(secret)
                    } else {
                        None
                    };

                (listen, secret)
            };
        if let Some(serde_json::Value::Object(experimental)) = map.get_mut("experimental") {
            if let Some(serde_json::Value::Object(clash_api)) = experimental.get_mut("clash_api") {
                init_clash_api_map(clash_api, false)
            } else {
                let mut clash_api_map = serde_json::Map::new();
                let (listen, secret) = init_clash_api_map(&mut clash_api_map, true);
                experimental.insert("clash_api".into(), serde_json::Value::Object(clash_api_map));
                (listen, secret)
            }
        } else {
            let mut experimental_map = serde_json::Map::new();
            let mut clash_api_map = serde_json::Map::new();
            let (listen, secret) = init_clash_api_map(&mut clash_api_map, true);
            experimental_map.insert("clash_api".into(), serde_json::Value::Object(clash_api_map));
            map.insert(
                "experimental".into(),
                serde_json::Value::Object(experimental_map),
            );
            (listen, secret)
        }
    }

    async fn cancel_and_wait(&mut self) {
        self.token.cancel();
        let _ = self.receiver.recv().await;
        self.status.clean_data();
        self.status.notify();
    }

    async fn child_handle(
        token: CancellationToken,
        _sender: mpsc::Sender<()>,
        mut child: Child,
        script_handler: super::ScriptHandler,
        log_queue: Arc<super::LogQueue<String>>,
        started_notify: Arc<Notify>,
        status: Arc<super::State<Status>>,
    ) {
        status.is_running.store(true, Ordering::Relaxed);
        status.notify();
        let mut stdout_buf_reader = BufReader::new(child.stdout.take().unwrap());
        let mut stderr_buf_reader = BufReader::new(child.stderr.take().unwrap());
        let mut stdout_string = String::new();
        let mut stderr_string = String::new();
        log_queue.push_data(format!(
            "[{}] service is started",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));
        loop {
            tokio::select! {
                res = stdout_buf_reader.read_line(&mut stdout_string) => {
                    if let Ok(_) = res {
                        if stdout_string.len() > 0 {
                            log::debug!("service: stdout: {}", stdout_string.trim_end());
                            let is_started_msg = stdout_string.contains("sing-box started");
                            log_queue.push_data(format!("[{}] stdout: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), stdout_string.trim_end()));
                            if is_started_msg {
                                script_handler.run_after_start_script().await;
                                log::debug!("service: core is started");
                                started_notify.notify_waiters();
                            }
                        }
                    }
                    stdout_string.clear();
                }
                res = stderr_buf_reader.read_line(&mut stderr_string) => {
                    if let Ok(_) = res {
                        if stderr_string.len() > 0 {
                            log::debug!("service: stderr: {}", stderr_string.trim_end());
                            let is_started_msg = stderr_string.contains("sing-box started");
                            log_queue.push_data(format!("[{}] stderr: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), stderr_string.trim_end()));
                            if is_started_msg {
                                script_handler.run_after_start_script().await;
                                log::debug!("service: core is started");
                                started_notify.notify_waiters();
                            }
                        }
                    }
                    stderr_string.clear();
                }
                res = child.wait() => {
                    if let Err(e) = res {
                        log::error!("service: service exited with error: {}", e);
                    }
                    break;
                }
                _ = token.cancelled() => {
                    script_handler.run_before_close_script().await;
                    log::debug!("service: service is cancelled");
                    Self::stop_process(child).await;
                    break;
                }
            }
        }
        script_handler.run_after_close_script().await;
        log_queue.push_data(format!(
            "[{}] service is closed",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));
        token.cancel();
        status.is_running.store(false, Ordering::Relaxed);
        status.notify();
    }

    async fn stop_process(mut child: Child) {
        use std::time;

        const WAIT_DURATION: time::Duration = time::Duration::from_secs(5);

        cfg_if::cfg_if! {
            if #[cfg(unix)] {
                use nix::unistd::Pid;
                use nix::sys::signal::{self, Signal};

                let pid = match child.id() {
                    Some(pid) => pid,
                    None => {
                        log::error!("service: failed to get pid");
                        return;
                    }
                };

                if let Err(_) = signal::kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
                    let _ = child.kill().await;
                    return;
                }

                tokio::select! {
                    _ = tokio::time::sleep(WAIT_DURATION) => {
                        let _ = child.kill().await;
                    },
                    _ = child.wait() => {}
                }
            } else if #[cfg(target_os = "windows")] {
                use windows::Win32::System::Console::{
                    FreeConsole, AttachConsole, SetConsoleCtrlHandler, GenerateConsoleCtrlEvent, CTRL_C_EVENT
                };

                let pid = match child.id() {
                    Some(pid) => pid,
                    None => {
                        log::error!("service: failed to get pid");
                        return;
                    }
                };

                unsafe {
                    let _ = FreeConsole();
                    let _ = AttachConsole(pid);
                    let _ = SetConsoleCtrlHandler(None, true);
                    let _ = GenerateConsoleCtrlEvent(CTRL_C_EVENT, pid);
                    let _ = SetConsoleCtrlHandler(None, false);
                    let _ = FreeConsole();
                    let _ = AttachConsole(u32::MAX);
                }

                tokio::select! {
                    _ = tokio::time::sleep(WAIT_DURATION) => {
                        let _ = child.kill().await;
                    },
                    _ = child.wait() => {}
                }
            } else {
                let _ = child.kill().await;
            }
        }
    }

    async fn clash_api_handle(
        listen: SocketAddr,
        secret: Option<String>,
        started_notify: Arc<Notify>,
        status: Arc<super::State<Status>>,
        token: CancellationToken,
        _sender: mpsc::Sender<()>,
    ) {
        // Wait Core Started
        tokio::select! {
            _ = started_notify.notified() => {}
            _ = token.cancelled() => {
                return;
            }
        }
        log::debug!("service: start clash api handle");
        let (listen_traffic, secret_traffic, token_traffic, status_traffic) = (
            listen.clone(),
            secret.clone(),
            token.clone(),
            status.clone(),
        );
        let (listen_speed, secret_speed, token_speed, status_speed) = (
            listen.clone(),
            secret.clone(),
            token.clone(),
            status.clone(),
        );
        let (listen_memory, secret_memory, token_memory, status_memory) =
            (listen, secret, token, status);
        let fut_traffic = async move {
            log::debug!("service: start clash api traffic handle");
            super::ClashAPIType::Traffic
                .handle::<_, super::ClashAPITrafficResult, _, _>(
                    listen_traffic,
                    secret_traffic,
                    token_traffic,
                    status_traffic,
                    |status, data| async move {
                        status.connection_count.store(
                            data.connections.map(|c| c.len()).unwrap_or(0),
                            Ordering::Relaxed,
                        );
                        status
                            .upload_traffic
                            .store(data.upload_traffic, Ordering::Relaxed);
                        status
                            .download_traffic
                            .store(data.download_traffic, Ordering::Relaxed);
                        status.notify();
                    },
                )
                .await;
            log::debug!("service: stop clash api traffic handle");
        };
        let fut_speed = async move {
            log::debug!("service: start clash api speed handle");
            super::ClashAPIType::Speed
                .handle::<_, super::ClashAPISpeedResult, _, _>(
                    listen_speed,
                    secret_speed,
                    token_speed,
                    status_speed,
                    |status, data| async move {
                        status
                            .upload_speed
                            .store(data.upload_speed, Ordering::Relaxed);
                        status
                            .download_speed
                            .store(data.download_speed, Ordering::Relaxed);
                        status.notify();
                    },
                )
                .await;
            log::debug!("service: stop clash api speed handle");
        };
        let fut_memory = async move {
            log::debug!("service: start clash api memory handle");
            super::ClashAPIType::Memory
                .handle::<_, super::ClashAPIMemoryResult, _, _>(
                    listen_memory,
                    secret_memory,
                    token_memory,
                    status_memory,
                    |status, data| async move {
                        status.memory_usage.store(data.memory, Ordering::Relaxed);
                        status.notify();
                    },
                )
                .await;
            log::debug!("service: stop clash api memory handle");
        };
        tokio::join!(fut_traffic, fut_speed, fut_memory);
    }
}

#[derive(Clone)]
pub(crate) struct Service {
    manager: Arc<Manager>,
    inner: Arc<Mutex<Option<ServiceInner>>>,
    log_queue: Arc<super::LogQueue<String>>,
    status: Arc<super::State<Status>>,
}

impl Service {
    pub(crate) fn new(manager: Arc<Manager>) -> Self {
        Self {
            manager,
            inner: Arc::new(Mutex::new(None)),
            log_queue: Arc::new(super::LogQueue::new(16)),
            status: Arc::new(super::State::new(
                Status::default(),
                Arc::new(Notify::new()),
            )),
        }
    }

    pub(crate) async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let db = self.manager.get_database();
        // Auto Start
        if let Ok(b) = database::get_auto_start(&db).await {
            if b {
                if let Err(e) = self.start_service().await {
                    log::error!("service: auto start failed: {}", e);
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn close(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Err(e) = self.stop_service().await {
            log::error!("service: close failed: {}", e);
        }
        Ok(())
    }

    async fn get_start_prepare_info(&self) -> Result<(String, database::Config), super::Error> {
        let db = self.manager.get_database();
        let core_path = match database::get_core_path(&db).await {
            Ok(Some(p)) => p,
            Ok(None) => return Err(super::Error::CorePathNotSet),
            Err(e) => return Err(super::Error::GetCorePathFailed(e.to_string())),
        };
        let config = match database::get_active_config(&db).await {
            Ok(Some(m)) => m,
            Ok(None) => return Err(super::Error::ConfigNotSet),
            Err(e) => return Err(super::Error::GetConfigFailed(e.to_string())),
        };
        Ok((core_path, config))
    }

    pub(crate) async fn start_service(&self) -> Result<(), super::Error> {
        self.restart_service().await
    }

    pub(crate) async fn stop_service(&self) -> Result<(), super::Error> {
        let mut inner_lock = self.inner.lock().await;
        if let Some(mut inner) = inner_lock.take() {
            inner.cancel_and_wait().await;
        }
        Ok(())
    }

    pub(crate) async fn restart_service(&self) -> Result<(), super::Error> {
        let mut inner_lock = self.inner.lock().await;
        if let Some(mut inner) = inner_lock.take() {
            inner.cancel_and_wait().await;
        }
        let (core_path, config) = self
            .get_start_prepare_info()
            .await
            .map_err(|e| super::Error::StartServiceFailed(e.to_string()))?;
        let inner = ServiceInner::new(
            self.manager.clone(),
            core_path,
            config,
            self.log_queue.clone(),
            self.status.clone(),
        )
        .await
        .map_err(|e| super::Error::StartServiceFailed(e.to_string()))?;
        inner_lock.replace(inner);
        Ok(())
    }

    pub(crate) async fn get_config(&self) -> Option<serde_json::Value> {
        self.inner
            .lock()
            .await
            .as_ref()
            .map(|inner| inner.config.config.clone())
    }

    pub(crate) fn log_queue_listener(&self) -> super::LogQueueListener<String> {
        self.log_queue.subscribe()
    }

    pub(crate) fn get_status(&self) -> (Arc<Notify>, &Status) {
        let notify = self.status.clone_notify();
        let status = self.status.as_ref();
        (notify, status)
    }
}

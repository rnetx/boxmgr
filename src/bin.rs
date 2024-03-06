extern crate boxmgr;

use std::{fs, net::SocketAddr, process::exit};

use boxmgr::manager::{Manager as boxManager, ManagerOptions};

use clap::Parser;
use tokio_util::sync::CancellationToken;

#[derive(Parser)]
#[command(version, about = "a sing-box manager", long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "config.json")]
    config: String,
}

#[derive(Debug, serde::Deserialize)]
struct Options {
    log_level: String,
    log_file: Option<String>,
    database_url: Option<String>,
    secret: String,
    listen: SocketAddr,
    local_listen_port: Option<u16>,
    data_dir: String,
    temp_dir: String,
}

impl Options {
    fn to_manager_options(self) -> Result<ManagerOptions, String> {
        let log_file = match self.log_file {
            Some(f) => match f.as_str() {
                "stdout" | "" => boxmgr::log::LogOutput::stdout(),
                "stderr" => boxmgr::log::LogOutput::stderr(),
                "off" => boxmgr::log::LogOutput::nop(),
                _ => boxmgr::log::LogOutput::file(&f).map_err(|e| e.to_string())?,
            },
            None => boxmgr::log::LogOutput::stdout(),
        };
        Ok(ManagerOptions {
            log_level: self.log_level,
            log_file,
            database_url: self.database_url,
            secret: self.secret,
            listen: self.listen,
            local_listen_port: self.local_listen_port,
            data_dir: self.data_dir.into(),
            temp_dir: self.temp_dir.into(),
        })
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_content = match fs::read_to_string(cli.config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            exit(1);
        }
    };
    let options: Options = match serde_json::from_str(&config_content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse config file: {}", e);
            exit(1);
        }
    };
    let options = match options.to_manager_options() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to prepare log output: {}", e);
            exit(1);
        }
    };
    let manager = match boxManager::prepare(options).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to prepare manager: {}", e);
            exit(1);
        }
    };
    let token = CancellationToken::new();
    let token_ctrlc = token.clone();
    ctrlc::set_handler(move || {
        token_ctrlc.cancel();
    })
    .unwrap();
    if let Err(e) = manager.run(token).await {
        eprintln!("Failed to run manager: {}", e);
        exit(1);
    }
}

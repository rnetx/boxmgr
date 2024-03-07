extern crate boxmgr;

mod windows;

use std::{fs, process::exit, sync::Arc};

use boxmgr::manager::{Manager as boxManager, ManagerOptions, ManagerRawOptions};

use clap::{Args, Parser, Subcommand};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Parser)]
#[command(version, about = "a sing-box manager", long_about = None)]
struct Cli {
    #[command(flatten)]
    args: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct GlobalArgs {
    #[clap(short, long, default_value = "config.json")]
    config: String,

    #[cfg(target_os = "windows")]
    #[clap(long)]
    as_windows_service: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run,

    #[cfg(target_os = "windows")]
    InstallService(InstallServiceArgs),

    #[cfg(target_os = "windows")]
    UninstallService,

    #[cfg(target_os = "windows")]
    StartService,

    #[cfg(target_os = "windows")]
    StopService,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Args)]
struct InstallServiceArgs {
    #[clap(long)]
    binary_path: Option<String>,

    #[clap(long)]
    config_path: Option<String>,

    #[clap(long)]
    auto_start: bool,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run => {
            cfg_if::cfg_if! {
                if #[cfg(target_os = "windows")] {
                    if cli.args.as_windows_service {
                        windows::run_as_service();
                    } else {
                        run_command(cli.args);
                    }
                } else {
                    run_command(cli.args);
                }
            }
        }

        #[cfg(target_os = "windows")]
        Commands::InstallService(args) => windows::install_windows_service(
            cli.args,
            args.binary_path,
            args.config_path,
            args.auto_start,
        ),

        #[cfg(target_os = "windows")]
        Commands::UninstallService => windows::uninstall_windows_service(),

        #[cfg(target_os = "windows")]
        Commands::StartService => windows::start_windows_service(),

        #[cfg(target_os = "windows")]
        Commands::StopService => windows::stop_windows_service(),
    }
}

async fn run_prepare(global_args: GlobalArgs) -> Arc<boxManager> {
    let config_content = match fs::read_to_string(global_args.config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            exit(1);
        }
    };
    let options: ManagerRawOptions = match serde_json::from_str(&config_content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse config file: {}", e);
            exit(1);
        }
    };
    let options = match ManagerOptions::try_from(options) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to prepare log output: {}", e);
            exit(1);
        }
    };
    match boxManager::prepare(options).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to prepare manager: {}", e);
            exit(1);
        }
    }
}

fn run_command(global_args: GlobalArgs) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async move {
        let manager = run_prepare(global_args).await;
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
    });
}

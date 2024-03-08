use clap::Parser;
use std::{
    error::Error,
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::exit,
    sync::mpsc,
    thread::sleep,
    time::{Duration, Instant},
};
use tokio_util::sync::CancellationToken;
use windows::Win32::Foundation::ERROR_SERVICE_DOES_NOT_EXIST;
use windows_service::{
    define_windows_service,
    service::{
        ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
        ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
    },
    service_control_handler::{register, ServiceControlHandlerResult},
    service_dispatcher,
    service_manager::{ServiceManager, ServiceManagerAccess},
};

const SERVICE_NAME: &str = "BoxManagerService";
const SERVICE_DISPLAY_NAME: &str = "BoxManager Service";

pub(super) fn install_windows_service(
    global_args: super::GlobalArgs,
    binary_path: Option<String>,
    config_path: Option<String>,
    auto_start: bool,
) {
    let config_path = PathBuf::from(config_path.unwrap_or({
        std::env::current_dir()
            .unwrap()
            .join(global_args.config)
            .to_string_lossy()
            .to_string()
    }));

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service manager: {}", e);
            exit(1);
        }
    };

    let service_binary_path = match std::env::current_exe() {
        Ok(v) => v,
        Err(e) => match binary_path {
            Some(v) => PathBuf::from(v),
            None => {
                eprintln!("Failed to get current executable path: {}", e);
                exit(1);
            }
        },
    };

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: {
            if auto_start {
                ServiceStartType::AutoStart
            } else {
                ServiceStartType::OnDemand
            }
        },
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![
            "--config".into(),
            config_path.into(),
            "--as-windows-service".into(),
            "run".into(),
        ],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    let service = match service_manager.create_service(&service_info, {
        ServiceAccess::CHANGE_CONFIG
            | ServiceAccess::QUERY_STATUS
            | ServiceAccess::START
            | ServiceAccess::STOP
            | ServiceAccess::DELETE
            | ServiceAccess::QUERY_CONFIG
    }) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create service: {}", e);
            exit(1);
        }
    };
    match service.set_description("BoxManager Service") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to set service description: {}", e);
            exit(1);
        }
    }

    println!("Service installed successfully");
}

pub(super) fn uninstall_windows_service() {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service manager: {}", e);
            exit(1);
        }
    };

    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
    let service = match service_manager.open_service(SERVICE_NAME, service_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service: {}", e);
            exit(1);
        }
    };

    if let Err(e) = service.delete() {
        eprintln!("Failed to delete service: {}", e);
        exit(1);
    }
    println!("Service try to delete successfully");

    let query_status = match service.query_status() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to query service status: {}", e);
            exit(1);
        }
    };
    if query_status.current_state != ServiceState::Stopped {
        // If the service cannot be stopped, it will be deleted when the system restarts.
        if let Err(e) = service.stop() {
            eprintln!("Failed to stop service: {}", e);
            exit(1);
        }

        println!("Stop service");
    }

    drop(service);

    let start = Instant::now();
    let timeout = Duration::from_secs(5);
    while start.elapsed() < timeout {
        if let Err(windows_service::Error::Winapi(e)) =
            service_manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS)
        {
            if e.raw_os_error() == Some(ERROR_SERVICE_DOES_NOT_EXIST.0 as i32) {
                println!("Service deleted successfully");
                return;
            }
        }
        sleep(Duration::from_secs(1));
    }
    println!("Service is marked for deletion");
}

pub(super) fn start_windows_service() {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service manager: {}", e);
            exit(1);
        }
    };

    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::START;
    let service = match service_manager.open_service(SERVICE_NAME, service_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service: {}", e);
            exit(1);
        }
    };

    let status = match service.query_status() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to query service status: {}", e);
            exit(1);
        }
    };
    if status.service_type == ServiceType::OWN_PROCESS {
        match service.start::<&OsStr>(&[]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to start service: {}", e);
                exit(1);
            }
        }
        println!("Service started successfully");
    } else {
        eprintln!("Service is not a process");
        exit(1);
    }
}

pub(super) fn stop_windows_service() {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service manager: {}", e);
            exit(1);
        }
    };

    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP;
    let service = match service_manager.open_service(SERVICE_NAME, service_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service: {}", e);
            exit(1);
        }
    };

    let status = match service.query_status() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to query service status: {}", e);
            exit(1);
        }
    };
    if status.service_type == ServiceType::OWN_PROCESS {
        match service.stop() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to stop service: {}", e);
                exit(1);
            }
        }
        println!("Service stopped successfully");
    } else {
        eprintln!("Service is not a process");
        exit(1);
    }
}

pub(super) fn get_windows_service_status() {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service manager: {}", e);
            exit(1);
        }
    };

    let service_access = ServiceAccess::QUERY_STATUS;
    let service = match service_manager.open_service(SERVICE_NAME, service_access) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open service: {}", e);
            exit(1);
        }
    };

    let status = match service.query_status() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to query service status: {}", e);
            exit(1);
        }
    };

    if status.current_state == ServiceState::Running {
        println!("Service is running");
    } else {
        println!("Service is not running");
    }
}

fn run_service(args: super::GlobalArgs) -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = CancellationToken::new();
    let token_event_handler = token.clone();
    let (tx, rx) = mpsc::channel::<()>();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                token_event_handler.cancel();
                rx.recv().ok();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = register(SERVICE_NAME, event_handler)
        .map_err(|e| format!("Failed to register service control handler: {}", e))?;

    let mut next_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    };

    status_handle
        .set_service_status(next_status.clone())
        .map_err(|e| format!("Failed to set service status: {}", e))?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async move {
        let _tx = tx;
        let manager = super::run_prepare(args).await;
        if let Err(e) = manager.run(token).await {
            eprintln!("Failed to run manager: {}", e);
        }
    });

    next_status.current_state = ServiceState::Stopped;

    status_handle
        .set_service_status(next_status)
        .map_err(|e| format!("Failed to set service status: {}", e))?;

    Ok(())
}

fn custom_service_main(_: Vec<OsString>) {
    let cli = super::Cli::parse();
    if let Err(e) = run_service(cli.args) {
        eprintln!("{}", e);
    }
}

define_windows_service!(ffi_service_main, custom_service_main);

pub(super) fn run_as_service() {
    if let Err(e) = service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
        eprintln!("Failed to start service dispatcher: {}", e);
        exit(1);
    }
}

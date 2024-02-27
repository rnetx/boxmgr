use std::{error::Error, path::PathBuf, sync::Arc};

use tokio::{fs, io::AsyncWriteExt, process::Command};

use crate::{common, database, manager::Manager};

pub(crate) struct ScriptHandler {
    manager: Arc<Manager>,
    before_start_script: Option<database::Script>,
    after_start_script: Option<database::Script>,
    before_close_script: Option<database::Script>,
    after_close_script: Option<database::Script>,
}

impl ScriptHandler {
    pub(crate) async fn new(manager: Arc<Manager>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db = manager.get_database();
        let (db_1, db_2, db_3) = (db.clone(), db.clone(), db.clone());
        let (result_before_start, result_after_start, result_before_close, result_after_close) = tokio::join!(
            database::get_before_start_script(&db_1),
            database::get_after_start_script(&db_2),
            database::get_before_close_script(&db_3),
            database::get_after_close_script(&db),
        );
        let before_start_script = match result_before_start {
            Ok(v) => v,
            Err(e) => return Err(format!("get before start script failed: {}", e).into()),
        };
        let after_start_script = match result_after_start {
            Ok(v) => v,
            Err(e) => return Err(format!("get after start script failed: {}", e).into()),
        };
        let before_close_script = match result_before_close {
            Ok(v) => v,
            Err(e) => return Err(format!("get before close script failed: {}", e).into()),
        };
        let after_close_script = match result_after_close {
            Ok(v) => v,
            Err(e) => return Err(format!("get after close script failed: {}", e).into()),
        };
        Ok(Self {
            manager,
            before_start_script,
            after_start_script,
            before_close_script,
            after_close_script,
        })
    }

    fn set_extension(path: &mut PathBuf) {
        #[cfg(unix)]
        path.set_extension("sh");

        #[cfg(target_os = "windows")]
        path.set_extension("bat");
    }

    fn bytes_to_string(bytes: Vec<u8>) -> String {
        String::from_utf8_lossy(&bytes).to_string()
    }

    async fn run_script(label: &str, manager: &Manager, script: &Option<database::Script>) {
        if let Some(script) = script {
            log::debug!("service: run {}: tag: {}", label, script.tag);
            let temp_script_file_name =
                label.replace(" ", "_") + "_" + common::random_uuid().replace("-", "").as_str();
            let mut temp_script_file = manager.get_temp_dir_path().join(temp_script_file_name);
            Self::set_extension(&mut temp_script_file);
            {
                let mut f = match fs::File::options()
                    .create(true)
                    .truncate(false)
                    .open(&temp_script_file)
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("service: create {} failed: {}", label, e);
                        return;
                    }
                };
                if let Err(e) = f.write_all(script.content.as_bytes()).await {
                    log::error!("service: write {} failed: {}", label, e);
                    return;
                }
            }

            // Set Permission
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let f = match std::fs::File::open(&temp_script_file) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("service: {}: set permission failed: {}", label, e);
                        return;
                    }
                };
                let metadata = match f.metadata() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("service: {}: set permission failed: {}", label, e);
                        return;
                    }
                };
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o755);
            }

            match Command::new(&temp_script_file).output().await {
                Ok(output) => {
                    let stdout = Self::bytes_to_string(output.stdout);
                    let stderr = Self::bytes_to_string(output.stderr);
                    let mut s = format!("exit code: {}", output.status);
                    if !stdout.is_empty() {
                        s.push_str(&format!("; stdout: {}", stdout));
                    }
                    if !stderr.is_empty() {
                        s.push_str(&format!("; stderr: {}", stderr));
                    }
                    log::debug!("service: run {} output: {}", label, s);
                }
                Err(e) => {
                    log::error!("service: run {} failed: {}", label, e);
                }
            }

            std::fs::remove_file(&temp_script_file).unwrap_or_else(|e| {
                log::error!("service: remove {} failed: {}", label, e);
            });
        }
    }

    pub(crate) async fn run_before_start_script(&self) {
        Self::run_script(
            "before start script",
            &self.manager,
            &self.before_start_script,
        )
        .await
    }

    pub(crate) async fn run_after_start_script(&self) {
        Self::run_script(
            "after start script",
            &self.manager,
            &self.after_start_script,
        )
        .await
    }

    pub(crate) async fn run_before_close_script(&self) {
        Self::run_script(
            "before close script",
            &self.manager,
            &self.before_close_script,
        )
        .await
    }

    pub(crate) async fn run_after_close_script(&self) {
        Self::run_script(
            "after close script",
            &self.manager,
            &self.after_close_script,
        )
        .await
    }
}

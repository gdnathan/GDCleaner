use crate::settings::Config;
use futures::future::{BoxFuture, FutureExt};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, OnceLock};
use tokio::fs::read_dir;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinSet;

static MAX_TASKS: OnceLock<Arc<Semaphore>> = OnceLock::new();
static DU_FAILED: OnceLock<()> = OnceLock::new();

pub fn sem() -> Arc<Semaphore> {
    Arc::clone(MAX_TASKS.get_or_init(|| Arc::new(Semaphore::new(64))))
}

pub struct Folder {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
}

pub struct IdentifiedFolder {
    pub project_name: String,
    pub language: String,
    pub folders: Vec<Folder>,
    pub size: u64,
}

pub async fn scan_folder(
    config: Arc<Config>,
    path: PathBuf,
    _permit: OwnedSemaphorePermit,
) -> Vec<IdentifiedFolder> {
    // let permit = MAX_TASKS.acquire().await.unwrap();
    let mut language_name = None;
    // Used so for example, a "target" folder in a node project will not be selected for removal
    // (path, file_name)
    let mut potential_targets: Vec<Folder> = vec![];
    let mut empty = true;

    match read_dir(&path).await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                empty = false;
                let file_name = entry.file_name().to_string_lossy().to_string();
                let size = get_size(&config, &entry.path());

                if let Some(name) = config.lang_identifier.get(&file_name) {
                    language_name = Some(name.clone());
                }
                if config.lang_target.get(&file_name).is_some() {
                    potential_targets.push(Folder {
                        path: entry.path(),
                        name: file_name.to_string(),
                        size: size,
                    })
                }
            }
        }
        Err(e) => {
            eprintln!("Could not open {:?}: {}", path.file_name(), e);
            return vec![];
        }
    }

    if language_name.is_none() {
        drop(_permit);
        return go_deeper(config, path).await;
    }

    if empty {
        drop(_permit);
        return vec![];
    }

    potential_targets.retain(|folder| config.lang_target.get(&folder.name).is_some());
    let size = potential_targets
        .iter()
        .fold(0, |size, folder| size + folder.size);
    vec![IdentifiedFolder {
        project_name: path
            .file_name()
            .expect("should be valid")
            .to_string_lossy()
            .to_string(),
        language: language_name.expect("checked"),
        folders: potential_targets,
        size,
    }]
}

fn go_deeper(config: Arc<Config>, path: PathBuf) -> BoxFuture<'static, Vec<IdentifiedFolder>> {
    async move {
        let mut set = JoinSet::new();

        let files = read_dir(&path).expect("checked in scan_folder");

        let subdirs: Vec<PathBuf> = files
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.file_type().map(|t| t.is_dir()).unwrap_or_default())
            .map(|f| f.path())
            .collect();

        for sd in subdirs {
            let config_clone = config.clone();
            let new_permit = sem().acquire_owned().await.unwrap();
            let _ = set.spawn({ scan_folder(config_clone, sd, new_permit) });
        }

        set.join_all()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    }
    .boxed()
}

fn get_size(config: &Arc<Config>, path: &PathBuf) -> u64 {
    if config.skip_size || DU_FAILED.get().is_some() {
        return 0;
    }
    Command::new("du")
        .arg("-sb")
        .arg(path)
        .output()
        .map(|e| {
            let stdout = String::from_utf8_lossy(&e.stdout);
            stdout
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
        })
        .unwrap_or_else(|e| {
            eprintln!("Error: Couldn't run du: {e}");
            let _ = DU_FAILED.set(());
            0
        })
}

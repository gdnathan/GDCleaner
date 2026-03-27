use crate::settings::Config;
use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use tokio::task::JoinSet;
use std::marker::Send;
use tokio::task;

static MAX_TASKS: OnceLock<Arc<Semaphore>> = OnceLock::new();

fn sem() -> Arc<Semaphore> {
    Arc::clone(MAX_TASKS.get_or_init(|| Arc::new(Semaphore::new(64))))
}

struct IdentifiedFolder {
    project_name: String,
    language: String,
    folders: Vec<(PathBuf, String)>,
    size: u64,
}
// todo: faire une mega hashmap pour les sources -> type
async fn scan_folder(config: Arc<Config>, path: PathBuf, _permit: OwnedSemaphorePermit) -> Vec<IdentifiedFolder> {
    // let permit = MAX_TASKS.acquire().await.unwrap();
    let mut language_name = None;
    // Used so for example, a "target" folder in a node project will not be selected for removal
    // (path, file_name)
    let mut potential_targets: Vec<(PathBuf, String)> = vec![];

    //TODO: change this to use tokio::fs::read_dir
    let try_files = read_dir(&path);
    match try_files {
        Ok(files) => {
            // TODO: with tokio's read_dir, this will be a poll and not an iterator. 
            // while let Some(entry) = dir.next_entry().await 
            files.for_each(|file| match file {
                Ok(file) => {
                    let file_name = file.file_name();
                    let file_name = file_name.to_string_lossy().to_string();
                    if let Some(name) = config.lang_identifier.get(&file_name) {
                        language_name = Some(name.clone());
                    }
                    if config.lang_target.get(&file_name).is_some() {
                        potential_targets.push((file.path(), file_name.to_string()))
                    }
                }
                Err(e) => {
                    eprintln!("ooooops something went wrong: {e}");
                }
            });
        }
        Err(e) => {
            eprintln!("Could not open {:?}: {}", path.file_name(), e);
            return vec!()
        }
    }

    if language_name.is_none() {
        drop(_permit);
        let e = go_deeper(config, path).await;
        return e
    }

    potential_targets.retain(|(_path, file_name)| config.lang_target.get(file_name).is_some());
    vec!(IdentifiedFolder {
        project_name: path.file_name().expect("should be valid").to_string_lossy().to_string(),
        language: language_name.expect("checked"),
        folders: potential_targets,
        size: 0
    })
}

async fn go_deeper(config: Arc<Config>, path: PathBuf) -> Vec<IdentifiedFolder> {
    // iter through folders and send a thread for each of them
    let mut identified_folders: Vec<IdentifiedFolder> = vec!();
    // let mut handles: Vec<JoinHandle<Vec<IdentifiedFolder>>> = vec!();
    let mut set = JoinSet::new();

    let files = read_dir(&path).expect("checked in scan_folder");

    let subdirs = unimplemented!("La je récupère tout les subdirs pour éviter le pb de PathDir");

    unimplemented!("Spawn all threads, waiting for a available permit");
    // for file in files {
    //     if let Ok(file) = file {
    //         if let Ok(file_type) = file.file_type() {
    //             let config_clone = config.clone();
    //             let sem = Arc::clone(&sem());
    //             // Faut ptet faire un Arc<Semaphor> dans ma config, comme ça je peux clone la
    //             let new_permit = sem.acquire_owned().await.unwrap();
    //             let file_path = file.path();
    //             if file_type.is_dir() {
    //                 let _ = set.spawn({
    //                     scan_folder(config_clone, file_path, new_permit)
    //                 });
    //
    //             }
    //         }
    //     }
    // }
    unimplemented!("Wait for all tasks to resolvee to aggregate the results");

    return identified_folders
}

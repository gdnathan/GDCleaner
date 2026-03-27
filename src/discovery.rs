use crate::settings::Config;
use std::fs::read_dir;
use std::path::PathBuf;
use tokio::sync::Semaphore;

static MAX_TASKS: Semaphore = Semaphore::const_new(64);

struct IdentifiedFolder {
    project_name: String,
    language: String,
    folders: Vec<(PathBuf, String)>,
    size: u64,
}
// todo: faire une mega hashmap pour les sources -> type
async fn scan_folder(config: &Config, path: &PathBuf) -> Vec<IdentifiedFolder> {
    let permit = MAX_TASKS.acquire().await.unwrap();
    let mut language_name = None;
    // Used so for example, a "target" folder in a node project will not be selected for removal
    // (path, file_name)
    let mut potential_targets: Vec<(PathBuf, String)> = vec![];

    let try_files = read_dir(path);
    match try_files {
        Ok(files) => {
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
        }
    }

    if language_name.is_none() {
        drop(permit);
        return go_deeper();
    }

    potential_targets.retain(|(_path, file_name)| config.lang_target.get(file_name).is_some());
    vec!(IdentifiedFolder {
        project_name: path.file_name().expect("should be valid").to_string_lossy().to_string(),
        language: language_name.expect("checked"),
        folders: potential_targets,
        size: 0
    })
}

fn go_deeper() -> Vec<IdentifiedFolder> {
    vec![]
}

use crate::settings::Config;
use std::fs::read_dir;
use std::path::PathBuf;
use tokio::sync::Semaphore;

static MAX_TASKS: Semaphore = Semaphore::const_new(64);

struct IdentifiedFolder {
    project_name: String,
    language: String,
    path: PathBuf,
    size: u64,
}
// todo: faire une mega hashmap pour les sources -> type
async fn scan_folder(config: &Config, path: &PathBuf) -> Vec<IdentifiedFolder> {
    let permit = MAX_TASKS.acquire().await.unwrap();
    let mut language_name = None;
    // Used so for example, a "target" folder in a node project will not be selected for removal
    let mut potential_targets: Vec<PathBuf> = vec![];

    let try_files = read_dir(path);
    match try_files {
        Ok(files) => {
            files.for_each(|file| match file {
                Ok(file) => {
                    let file_name = file.file_name();
                    let file_name = file_name.to_str().expect("should always be valid utf8").clone();
                    if let Some(name) = config
                        .lang_identifier
                        .get(file_name)
                    {
                        language_name = Some(name.clone());
                    }
                    if config.all_targets.get(file_name).is_some() {
                        potential_targets.push(file.path())
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
    drop(permit);
    // la on check si y'a un language identifié.
    // si oui, on filtre les target, et on retourne un vec de target avec les infos
    // si non, on lance une task pour tout les folder dispo, et on aggregate les résultats pour les
    // retourner

    return vec![];
}

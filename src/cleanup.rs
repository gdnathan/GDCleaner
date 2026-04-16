use crate::discovery::IdentifiedFolder;
use crate::settings::Config;

pub async fn start_cleanup(config: &Config, project_folders: Vec<IdentifiedFolder>) -> u64 {
    let mut cleared_size = 0;
    for project in project_folders {
        if project.folders.is_empty() {
            continue;
        }
        println!("🗑️ Deleting folders for project {}", project.project_name);
        'folder: for folder in project.folders {
            if config.force {
                match std::fs::remove_dir_all(folder.path) {
                    Err(e) => eprintln!("❌ Error: Could not remove {} : {:?}", folder.name, e),
                    Ok(_) => cleared_size += folder.size,
                }
                continue 'folder;
            }

            'user: loop {
                let mut s = String::new();
                println!("❓ Delete \"{}\" ? [Y/n/path]", folder.name);
                if let Err(e) = std::io::stdin().read_line(&mut s) {
                    eprintln!("❌ Error: Could not read input: {}", e);
                    continue 'user;
                }
                s = s.to_lowercase();
                match &*s {
                    "y\n" | "\n" => {
                        break 'user;
                    }
                    "n\n" => {
                        continue 'folder;
                    }
                    "path\n" => {
                        println!("💡 Path: {}", folder.path.display());
                        continue 'user;
                    }
                    _ => {
                        println!("❌ Error: Invalid input");
                        continue 'user;
                    }
                }
            }

            match std::fs::remove_dir_all(&folder.path) {
                Err(e) => {
                    if folder.path.is_file() {
                        match std::fs::remove_file(folder.path) {
                            Err(e) => {
                                eprintln!("❌ Error: Could not remove {} : {:?}", folder.name, e);
                            }
                            Ok(_) => cleared_size += folder.size,
                        }
                    } else {
                        eprintln!("❌ Error: Could not remove {} : {:?}", folder.name, e);
                    }
                }
                Ok(_) => cleared_size += folder.size,
            }
        }
    }

    cleared_size
}

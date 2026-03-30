mod settings;
use settings::generate_config;

mod discovery;
use discovery::{IdentifiedFolder, scan_folder, sem};

use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let config = generate_config();

    let chronometer = Instant::now();
    let permit = sem().acquire_owned().await.unwrap();
    let path = config.path.clone();
    let project_folders = scan_folder(Arc::new(config.clone()), path, permit).await;

    if config.verbose {
        print_discovery_info(&project_folders);
    }

    println!(
        "💡 Discovered {} projects to clean in {} ms \n",
        project_folders.len(),
        chronometer.elapsed().as_millis()
    );

    let mut cleared_size = 0;
    for project in project_folders {
        if project.folders.is_empty() {
            continue
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
                        continue 'user
                    }
                    _ => {
                        println!("❌ Error: Invalid input");
                        continue 'user;
                    }
                }
            }
            match std::fs::remove_dir_all(folder.path) {
                Err(e) => eprintln!("❌ Error: Could not remove {} : {:?}", folder.name, e),
                Ok(_) => cleared_size += folder.size,
            }
        }
    }

    println!("✅ Cleared size: {} bytes !", cleared_size);
}

fn print_discovery_info(project_folders: &Vec<IdentifiedFolder>) {
    println!("folders to delete:");
    println!("#######################");
    for f in project_folders {
        println!("Project name: {}", f.project_name);
        println!("Clearable size: {}", f.size);
        println!("Identified language{}", f.language);
        println!("folders");
        for ff in &f.folders {
            println!("------------------");
            println!("folder name: {}", ff.name);
            println!("folder path: {:?}", ff.path);
        }
        println!("#######################");
    }
}

mod settings;
use settings::generate_config;

mod discovery;
use discovery::{IdentifiedFolder, scan_folder, sem};

mod cleanup;
use cleanup::start_cleanup;

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

    let cleared_size = start_cleanup(&config, project_folders).await;

    let display = if cleared_size >= 1_000_000_000 {
        format!("{:.2} GB", cleared_size as f64 / 1_000_000_000.0)
    } else if cleared_size >= 1_000_000 {
        format!("{:.2} MB", cleared_size as f64 / 1_000_000.0)
    } else if cleared_size >= 1_000 {
        format!("{:.2} KB", cleared_size as f64 / 1_000.0)
    } else {
        format!("{} bytes", cleared_size)
    };

    println!("✅ Cleared size: {}", display);
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

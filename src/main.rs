mod settings;
use settings::{Config, generate_config};

mod discovery;
use discovery::{scan_folder, IdentifiedFolder, sem};

use std::sync::Arc;
use tokio::runtime;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let config = generate_config();
    // println!("{:?}", config);

    let chronometer = Instant::now();
    let permit = sem().acquire_owned().await.unwrap();
    let path = config.path.clone();
    let folders_to_delete = scan_folder(Arc::new(config.clone()), path, permit).await;

    if config.verbose {
        print_discovery_info(&folders_to_delete);
    }

    println!("Discovered {} projects to clean in {} ms", folders_to_delete.len(), chronometer.elapsed().as_millis());

}

fn print_discovery_info(folders_to_delete: &Vec<IdentifiedFolder>) {
    println!("folders to delete:");
    println!("#######################");
    for f in folders_to_delete {
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

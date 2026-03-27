mod settings;
use settings::{Config, generate_config};

mod discovery;

fn main() {
    let config = generate_config();
    println!("{:?}", config);


}


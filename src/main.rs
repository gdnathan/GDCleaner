mod settings;
use settings::{Config, generate_config};


fn main() {
    let config = generate_config();
    println!("{:?}", config);


}


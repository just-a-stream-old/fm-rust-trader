mod config;

use crate::config::*;

fn main() {
    let config = load_config();

    println!("{:#?}", config);

}

mod handlers;
mod utils;

use crate::handlers::config::CompleteConfig;

fn main() {
    match CompleteConfig::new() {
        Ok(config) => {
            println!("Hello World!");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

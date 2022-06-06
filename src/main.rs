use color_eyre::eyre::{Error, Result, WrapErr};

mod handlers;
mod utils;

use crate::handlers::config::CompleteConfig;

fn main() -> Result<(), Error> {
    color_eyre::install().unwrap();

    let mut _config = CompleteConfig::new()
        .wrap_err("Unable to read configuration file.")
        .unwrap();

    Ok(())
}

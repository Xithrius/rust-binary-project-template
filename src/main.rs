#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::future_not_send)]

use color_eyre::eyre::{Error, Result, WrapErr};

mod handlers;
mod utils;

use crate::handlers::config::CompleteConfig;

fn main() -> Result<(), Error> {
    color_eyre::install()?;

    let mut _config = CompleteConfig::new().context("Unable to read configuration file.")?;

    std::process::exit(0)
}

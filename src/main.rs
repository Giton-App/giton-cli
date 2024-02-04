use core::db;
use std::process::Command;

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use utils::app_config::AppConfig;
use utils::error::{Error, Result};

/// The main entry point of the application.
fn main() -> Result<()> {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Better Panic. Only enabled *when* debugging.
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    let _guard = utils::logger::setup_logging()?;

    // Initialize Configuration
    let config_contents = include_str!("resources/default_config.toml");
    AppConfig::init(Some(config_contents))?;

    // Throw error if no OpenAI key is set
    if AppConfig::get::<String>("openai_key")? == "" {
        println!("No OpenAI API key set. Please set one with `export GITON_OPENAI_KEY=<key>`");
        return Ok(());
    }

    // Match Commands
    let cli_match = cli::cli_match();

    dbg!(&cli_match);

    match cli_match {
        Ok(_) => {}
        Err(e) => {
            // extract args
            let args: Vec<String> = std::env::args().skip(1).collect();
            let args_string: String = args.join(" ");

            Command::new("git")
                .args(args)
                .spawn()
                .expect("git command failed to start");

            // if success, insert command into file
            db::insert_command(args_string)?;
        }
    }

    Ok(())
}

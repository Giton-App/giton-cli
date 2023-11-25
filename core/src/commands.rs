use utils::app_config::AppConfig;
use utils::error::Result;

use crate::db;

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;
    println!("{:#?}", config);

    Ok(())
}

pub fn history() -> Result<()> {
    db::display_commands()?;

    Ok(())
}

pub fn undo() -> Result<()> {
    let last_command = db::get_last_command()?;

    println!("Undoing: {}", last_command);

    Ok(())
}

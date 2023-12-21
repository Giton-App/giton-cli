extern crate log;

pub mod commands;
pub mod db;
pub mod decode;
pub mod git;

use utils::error::Result;

static PROMPT_UNDO: &str = include_str!("../prompts/undo.txt");
static PROMPT_HELPME: &str = include_str!("../prompts/helpme.txt");

pub fn start() -> Result<()> {
    Ok(())
}

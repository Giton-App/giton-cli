use std::{
    fs::OpenOptions,
    io::{BufRead, Write},
    time::{SystemTime, UNIX_EPOCH},
};
use utils::error::Result;

// Insert command into file (.giton) in a new line.
pub fn insert_command(command: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".giton")?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let line = format!("{}, {}\n", timestamp, command);

    file.write_all(line.as_bytes())?;

    Ok(())
}

pub fn get_commands() -> Result<Vec<String>> {
    let file = OpenOptions::new().read(true).open(".giton")?;

    let commands = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    Ok(commands)
}

pub fn get_last_command() -> Result<String> {
    let file = OpenOptions::new().read(true).open(".giton")?;

    let last_command = std::io::BufReader::new(file)
        .lines()
        .last()
        .unwrap()
        .unwrap();

    Ok(last_command)
}

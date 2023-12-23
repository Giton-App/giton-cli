use std::{
    fs::OpenOptions,
    io::{BufRead, Write},
    time::{SystemTime, UNIX_EPOCH},
};
use utils::error::{Error, Result};

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

// Get all commands from file (.giton) and return them as a Vec<String>.
pub fn get_commands() -> Result<Vec<String>> {
    let file = OpenOptions::new().read(true).open(".giton")?;

    let commands = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or(String::from("")))
        .collect();

    Ok(commands)
}

// Get the last command from file (.giton) and return it as a String.
pub fn get_last_command() -> Result<String> {
    let file = OpenOptions::new().read(true).open(".giton")?;

    let last_line = std::io::BufReader::new(file)
        .lines()
        .last()
        .ok_or_else(|| Error::new("No commands found"))??;

    let last_command = last_line.split(", ").collect::<Vec<&str>>()[1].to_string();

    Ok(last_command)
}

// Display all commands from file (.giton) in a human-readable format for the command line.
pub fn display_commands() -> Result<()> {
    let commands = get_commands()?;

    for command in commands {
        let timestamp = command.split(", ").collect::<Vec<&str>>()[0];
        let command = command.split(", ").collect::<Vec<&str>>()[1];

        let timestamp = timestamp.parse::<u64>()?;
        let datetime = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);

        // use chrono to convert datetime to user's timezone
        let datetime =
            chrono::DateTime::<chrono::Local>::from(datetime).format("%Y-%m-%d %H:%M:%S");

        println!("{}: {}", datetime, command);
    }

    Ok(())
}

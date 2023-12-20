use std::process::Command;
use utils::error::Result;

// Get Git status and return it as a String
pub fn get_status() -> Result<String> {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    let output_string = String::from_utf8(output.stdout)?;

    Ok(output_string)
}

// Get Git log and return it as a String. Only return the first 10 lines. Lines are separated by \n.
pub fn get_log() -> Result<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()?
        .stdout;

    // limit the output to 10 lines
    let output_lines = output.split(|&c| c == b'\n').take(10).collect::<Vec<_>>();

    // join the lines with \n
    let lines_string = String::from_utf8(output_lines.join(&b'\n'))?;

    Ok(lines_string)
}

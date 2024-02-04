use std::process::Command;
use utils::error::{GitonError, Result};

use crate::decode::{GPTResponse, ResponseStatus};

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

pub fn execute_command(command: String) -> Result<()> {
    // create git command
    let mut output = Command::new("git");

    // split command string into args
    shlex::split(&command).unwrap().iter().for_each(|arg| {
        output.arg(arg);
    });

    // execute command
    let output_stdout = output.output().expect("failed to execute process").stdout;

    // convert output to String
    let output = String::from_utf8(output_stdout)?;

    // store command to db
    crate::db::insert_command(command)?;

    // print output
    println!("{}", output);

    Ok(())
}

pub fn execute_gptresponse(response: GPTResponse) -> Result<()> {
    match response.status {
        ResponseStatus::Success => {
            response.commands.iter().for_each(|command| {
                execute_command(command.to_string()).unwrap();
            });

            return Ok(());
        }
        _ => {
            return Err(GitonError::new("GPTResponse cannot be executed"));
        }
    }
}

use comfy_table::Table;
use utils::error::{GitonError, Result};

#[derive(Debug)]
pub enum GPTResult {
    Success(GPTResponse),
    Failure(String),
}

#[derive(Debug)]
pub struct GPTResponse {
    pub status: ResponseStatus,
    pub explanation: String,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub enum ResponseStatus {
    Success,
    NotValid,
    NotPossible,
}

pub fn decode_gpt_response(response: String) -> Result<GPTResult> {
    let response = response.trim();

    if response.starts_with("NOT VALID:") {
        let explanation = response.replace("NOT VALID:", "");

        return Ok(GPTResult::Success(GPTResponse {
            status: ResponseStatus::NotValid,
            explanation,
            commands: vec![],
        }));
    }

    if response.starts_with("NOT POSSIBLE:") {
        let explanation = response.replace("NOT POSSIBLE:", "");

        return Ok(GPTResult::Success(GPTResponse {
            status: ResponseStatus::NotPossible,
            explanation,
            commands: vec![],
        }));
    }

    if response.starts_with("git") {
        // Sanitize commands
        let commands: Vec<String> = response
            .split("&&")
            .map(|command| {
                if command.trim().starts_with("git") {
                    Ok(command.trim().replace("git ", ""))
                } else {
                    Err(GitonError::new(
                        "One or more commands do not start with 'git'",
                    ))
                }
            })
            .collect::<Result<Vec<String>>>()?;

        return Ok(GPTResult::Success(GPTResponse {
            status: ResponseStatus::Success,
            explanation: String::from(""),
            commands,
        }));
    }

    Err(GitonError::DecodeResponse(response.to_string()))
}

// implement display for GPTResult
// use comfy_table::Table to display the commands
impl std::fmt::Display for GPTResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPTResult::Success(response) => {
                let mut table = Table::new();

                table.set_header(vec!["#", "Command"]);

                response
                    .commands
                    .iter()
                    .enumerate()
                    .for_each(|(i, command)| {
                        table.add_row(vec![i.to_string(), format!("git {}", command.to_string())]);
                    });

                write!(f, "{}", table)
            }
            GPTResult::Failure(error) => write!(f, "{}", error),
        }
    }
}

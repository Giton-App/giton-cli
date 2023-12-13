use std::process::Command;

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};
use async_openai::Client;
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

// run git status and return the output as a string
pub fn get_git_status() -> Result<String> {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();

    Ok(output)
}

// run git log and return the output as a String
pub fn get_git_log() -> Result<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .expect("failed to execute process");

    // limit the output to 10 lines
    let cut_output = output.stdout.split_at(1000).0;
    let cut_output_str = String::from_utf8(cut_output.to_vec()).unwrap();

    Ok(cut_output_str)
}

pub fn undo() -> Result<()> {
    let last_command = db::get_last_command()?;
    let git_status = get_git_status()?;
    let git_log = get_git_log()?;

    println!("Undoing: {}", last_command);

    //println!("Undoing: {}", last_command);
    let openai_key = AppConfig::fetch()?.openai_key;
    let config = OpenAIConfig::new().with_api_key(openai_key);

    let client = Client::with_config(config);

    let request = async_openai::types::CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(format!(
                    "You are an assistant that produces Git commands. The command you produce reverses the command you are given. 

IMPORTANT:
- ONLY RETURN THE CLI COMMAND IN BASH.
- DO NOT ADD ```bash ```
- IF THE COMMAND IS NOT REVESIBLE, RETURN 'NOT REVERSIBLE'
- IF THE COMMAND IS NOT VALID, RETURN 'NOT VALID'
- IF YOU CAN EXPLAIN 'NOT REVERSIBLE' OR 'NOT VALID', DO SO.
- FORMAT: 'NOT REVERSIBLE: <EXPLANATION>'

GIT STATUS:
{}
GIT LOG:
{}
",
                    git_status, git_log
                ))
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!(
                    "git {}",
                    last_command
                ))
                .build()
                .unwrap()
                .into(),
        ])
        .max_tokens(50_u16)
        .build()
        .unwrap();

    let response = async_std::task::block_on(
        client
            .chat() // Get the API "group" (completions, images, etc.) from the client
            .create(request), // Make the API call in that "group"
    )
    .unwrap();

    println!("{:#?}", response);

    Ok(())
}

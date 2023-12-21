use std::process::Command;

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};
use async_openai::Client;
use utils::app_config::AppConfig;
use utils::error::{Error, Result};

use comfy_table::Table;
use spinners::{Spinner, Spinners};

use crate::db;
use crate::decode::{GPTResponse, GPTResult};
use crate::git::{self, execute_gptresponse};

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;

    let mut table = Table::new();

    table.set_header(vec!["Key", "Value"]);

    config.into_iter().for_each(|(key, value)| {
        table.add_row(vec![key, value]);
        return ();
    });

    println!("{}", table);

    Ok(())
}

pub fn history() -> Result<()> {
    db::display_commands()?;

    Ok(())
}

pub fn undo() -> Result<()> {
    let mut spinner = Spinner::new(Spinners::Dots2, "Communicating with Open AI".into());

    let last_command = db::get_last_command()?;
    let git_status = git::get_status()?;
    let git_log = git::get_log()?;

    let openai_key = AppConfig::fetch()?.openai_key;
    let config = OpenAIConfig::new().with_api_key(openai_key);

    let client = Client::with_config(config);

    let prompt = crate::PROMPT_UNDO
        .replace("{git_status}", &git_status)
        .replace("{git_log}", &git_log);

    let request = async_openai::types::CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!("git {}", last_command))
                .build()?
                .into(),
        ])
        .max_tokens(50_u16)
        .build()?;

    let response = async_std::task::block_on(
        client
            .chat() // Get the API "group" (completions, images, etc.) from the client
            .create(request), // Make the API call in that "group"
    )?;

    spinner.stop();
    println!("");

    let returned_command = response
        .choices
        .first()
        .ok_or_else(|| Error::new("No choices returned"))?
        .message
        .content
        .as_ref()
        .ok_or_else(|| Error::new("No content returned"))?;

    let decoded_command = crate::decode::decode_gpt_response(returned_command.to_string())?;

    println!("{}", &decoded_command);

    let gpt_response: GPTResponse = match decoded_command {
        GPTResult::Success(gpt_response) => gpt_response,
        GPTResult::Failure(msg) => {
            println!("Giton failed. You can try again. \n {}", &msg);

            return Ok(());
        }
    };

    println!(":: Prooced with Command(s)?: \n [Y/n]");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim() == "Y" {
        // execute GPTResponse
        execute_gptresponse(gpt_response)?;
    }

    Ok(())
}

pub fn helpme() -> Result<()> {
    let mut spinner = Spinner::new(Spinners::Dots2, "Communicating with Open AI".into());

    let git_status = git::get_status()?;
    let git_log = git::get_log()?;

    let openai_key = AppConfig::fetch()?.openai_key;
    let config = OpenAIConfig::new().with_api_key(openai_key);

    let client = Client::with_config(config);

    let prompt = crate::PROMPT_HELPME
        .replace("{git_status}", &git_status)
        .replace("{git_log}", &git_log);

    let request = async_openai::types::CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages([ChatCompletionRequestSystemMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .max_tokens(50_u16)
        .build()?;

    let response = async_std::task::block_on(
        client
            .chat() // Get the API "group" (completions, images, etc.) from the client
            .create(request), // Make the API call in that "group"
    )?;

    spinner.stop();
    println!("");

    let returned_command = response
        .choices
        .first()
        .ok_or_else(|| Error::new("No choices returned"))?
        .message
        .content
        .as_ref()
        .ok_or_else(|| Error::new("No content returned"))?;

    let decoded_command = crate::decode::decode_gpt_response(returned_command.to_string())?;

    println!("{}", &decoded_command);

    let gpt_response: GPTResponse = match decoded_command {
        GPTResult::Success(gpt_response) => gpt_response,
        GPTResult::Failure(msg) => {
            println!("Giton failed. You can try again. \n {}", &msg);

            return Ok(());
        }
    };

    println!(":: Prooced with Command(s)?: \n [Y/n]");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim() == "Y" {
        execute_gptresponse(gpt_response)?;
    }

    Ok(())
}

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

pub fn undo() -> Result<()> {
    let last_command = db::get_last_command()?;

    //println!("Undoing: {}", last_command);
    let config =
        OpenAIConfig::new().with_api_key("");

    let client = Client::with_config(config);

    let request = async_openai::types::CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(
                    "You are an assistant that produces Git commands.

IMPORTANT:
- ONLY RETURN THE CLI COMMAND IN BASH.
- DO NOT ADD ```bash ```
- IF THE COMMAND IS NOT REVESIBLE, RETURN 'NOT REVERSIBLE'
- IF THE COMMAND IS NOT VALID, RETURN 'NOT VALID'",
                )
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!("Reverse this git command: {}", last_command))
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

    println!("{:?}", response);

    Ok(())
}

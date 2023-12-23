use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{
    generate,
    shells::{Bash, Fish, Zsh},
};
use std::{path::PathBuf, str::FromStr};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;
use utils::types::LogLevel;

#[derive(Parser, Debug)]
#[command(name = "giton", author, about, long_about = "Giton CLI", version)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub onconfig: Option<PathBuf>,

    /// Set a custom config file
    #[arg(name = "debug", short, long = "debug", value_name = "DEBUG")]
    pub debug: Option<bool>,

    /// Set Log Level
    #[arg(
        name = "log_level",
        short,
        long = "log-level",
        value_name = "LOG_LEVEL"
    )]
    pub log_level: Option<LogLevel>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(
        name = "undo",
        about = "Undo last command",
        long_about = None, 
    )]
    Undo,
    #[clap(
        name = "helpme",
        about = "Help Me!",
        long_about = None, 
    )]
    HelpMe,
    #[clap(
        name = "history",
        about = "Show a history of Git command executed",
        long_about = None, 
    )]
    History,
    #[clap(
        name = "completion",
        about = "Generate completion scripts",
        long_about = None,
        )]
    Completion {
        #[clap(subcommand)]
        subcommand: CompletionSubcommand,
    },
    #[clap(
        name = "onconfig",
        about = "Show Giton Configuration",
        long_about = None,
    )]
    Config,
}

#[derive(Subcommand, PartialEq, Debug)]
enum CompletionSubcommand {
    #[clap(about = "generate the autocompletion script for bash")]
    Bash,
    #[clap(about = "generate the autocompletion script for zsh")]
    Zsh,
    #[clap(about = "generate the autocompletion script for fish")]
    Fish,
}

pub fn cli_match() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::try_parse()?;

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli.onconfig.as_deref())?;

    let app = Cli::command();
    let matches = app.get_matches();

    AppConfig::merge_args(matches)?;

    // Execute the subcommand
    match &cli.command {
        Commands::Undo => commands::undo()?,
        Commands::HelpMe => commands::helpme()?,
        Commands::History => commands::history()?,
        Commands::Completion { subcommand } => {
            let mut app = Cli::command();
            match subcommand {
                CompletionSubcommand::Bash => {
                    generate(Bash, &mut app, "giton", &mut std::io::stdout());
                }
                CompletionSubcommand::Zsh => {
                    generate(Zsh, &mut app, "giton", &mut std::io::stdout());
                }
                CompletionSubcommand::Fish => {
                    generate(Fish, &mut app, "giton", &mut std::io::stdout());
                }
            }
        }
        Commands::Config => commands::config()?,
    }

    Ok(())
}

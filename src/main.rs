#![warn(clippy::all)]
use std::{env, error::Error};

use api::Openai;
use clap::{Parser, Subcommand};

mod api;
mod commands;

#[derive(Parser)]
#[command(name = "gptut")]
#[command(version = "0.0.1")]
#[command(author = "Serghei Mateas <matiaspub@gmail.com>")]
#[command(about = "A simple CLI app to chat with OpenAI models")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "chat with selected model")]
    Chat,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let api_key = env::var("OPENAI_API_KEY").expect("env OPENAI_API_KEY");
    let api = Openai::new("https://api.openai.com".to_string(), api_key)?;

    match cli.cmd {
        Commands::Chat => commands::chat(api).await?,
    };

    Ok(())
}

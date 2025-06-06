use anyhow::Error;
use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use mail::MailClient;
use serde_json::Value;

mod cli;
mod mail;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().unwrap();
    let args = Cli::parse();
    let client = MailClient::new(std::env::var("API_KEY").unwrap());

    match args {
        Cli::Mail => {
            println!("{:#?}", client.get_mail().await);
        }
        Cli::View { id } => {}
        Cli::Package { id } => {}
    }
    Ok(())
}

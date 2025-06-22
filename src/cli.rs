use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone)]
pub enum MailType {
    Legacy,
    Letter,
    Package,
}

#[derive(Parser)]
pub enum Cli {
    Mail {
        #[arg(short, long)]
        r#type: Option<MailType>,
    },
    View {
        #[arg(short, long)]
        id: String,
    },
    Fetch,
    Setup {
        api_key: String,
    },
    Credit,
    Fun,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { api_key: "".into() }
    }
}

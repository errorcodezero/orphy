use clap::{Parser, ValueEnum};

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
}

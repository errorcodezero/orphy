use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    Mail,
    View {
        #[arg(short, long)]
        id: String,
    },
    Fetch,
}

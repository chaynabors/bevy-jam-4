use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long)]
    pub server: Option<String>,
}

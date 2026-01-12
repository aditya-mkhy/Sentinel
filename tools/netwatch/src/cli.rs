use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "netwatch")]
pub struct Args {
    /// Output result in JSON format
    #[arg(long)]
    pub json: bool,
}
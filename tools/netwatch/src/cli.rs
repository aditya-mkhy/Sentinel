use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "netwatch")]
pub struct Args {
    /// Output result in JSON format
    #[arg(long)]
    pub json: bool,

    /// Resolve remote IPs to domain names (uses cache)
    #[arg(long)]
    pub resolve: bool,

    /// Re-resolve all DNS entries, ignoring cache
    #[arg(long = "resolve-refresh")]
    pub resolve_refresh: bool,
}
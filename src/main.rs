mod commands;

use crate::commands::Command;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(if args.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_target(args.debug)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(args.command.execute())
}

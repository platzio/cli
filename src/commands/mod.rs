mod chart;

use anyhow::Result;

#[derive(clap::Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Chart(chart::ChartCommand),
}

impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Chart(chart_command) => chart_command.execute().await,
        }
    }
}

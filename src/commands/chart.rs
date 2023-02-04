use anyhow::{anyhow, Result};
use platz_chart_ext::ChartExt;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(clap::Subcommand)]
pub enum ChartCommand {
    #[command(name = "lint")]
    Lint {
        #[arg(default_value = ".")]
        chart_path: PathBuf,
    },
}

impl ChartCommand {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Lint { chart_path } => {
                info!("Linting chart in {:?}", chart_path);
                let chart_ext = ChartExt::from_path(&chart_path).await?;
                match chart_ext.error {
                    None => {
                        info!("👍 Chart has no errors");
                        Ok(())
                    }
                    Some(error) => {
                        error!("Found errors while linting chart:");
                        error!("* {}", error);
                        Err(anyhow!("Had errors while parsing chart"))
                    }
                }
            }
        }
    }
}

pub mod export;
use std::error::Error;

use export::export_stops_to_excel;

use crate::cli::{Cli, ModelCommand, StopCommand};
use crate::config::Config;

pub async fn run(cli: Cli, config: &Config) -> Result<(), Box<dyn Error>> {
    match cli.model {
        ModelCommand::Stop(cmd) => match cmd {
            StopCommand::Export(args) => {
                export_stops_to_excel(args.file_name, config).await?;
            }
            StopCommand::Format(_) => {}
        },
    }
    Ok(())
}

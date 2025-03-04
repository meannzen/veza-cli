pub mod stop;
use std::error::Error;

use stop::{process_export_stops_to_excel, process_format_command};

use crate::cli::{Cli, ModelCommand, StopCommand};
use crate::config::Config;

pub async fn run(cli: Cli, config: &Config) -> Result<(), Box<dyn Error>> {
    match cli.model {
        ModelCommand::Stop(cmd) => match cmd {
            StopCommand::Export(args) => {
                process_export_stops_to_excel(args.file_name, config).await?;
            }
            StopCommand::Format(format_command) => {
                process_format_command(format_command, config).await?
            }
        },
    }
    Ok(())
}

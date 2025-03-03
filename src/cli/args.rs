use clap::{Args, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    author = "Sen Meann senmean@gmail.com",
    version = "1.0.0",
    about = "Menage data model with Excel and format"
)]
pub struct Cli {
    #[command(subcommand)]
    pub model: ModelCommand,
}

#[derive(Subcommand, Debug)]
pub enum ModelCommand {
    #[command(subcommand)]
    Stop(StopCommand),
}

#[derive(Subcommand, Debug)]
pub enum StopCommand {
    Export(ExportArgs),
    Format(FormatArgs),
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    #[arg(short = 'f', long = "file", default_value = "output.xlsx")]
    pub file_name: String,
}

#[derive(Args, Debug)]
pub struct FormatArgs {
    #[arg(short = 'u', long = "update-backend", default_value_t = false)]
    pub update_backend: bool,
}

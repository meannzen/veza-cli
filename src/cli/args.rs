use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Sen Meann senmean@gmail.com",
    version = "1.0.0",
    about = "Manage data model with Excel and format"
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
    /// Exports stop data to an Excel file.
    Export(ExportArgs),
    /// Formats stop data from different sources and optionally writes to an Excel file.
    #[command(subcommand)]
    Format(FormatCommand),
}

#[derive(Subcommand, Debug)]
pub enum FormatCommand {
    /// Pulls stops from the backend API, formats them, and optionally writes to an Excel file.
    Pull(PullFormatArgs),
    /// Reads stops from an Excel file, formats them, and optionally writes to a new Excel file.
    ReadXlsx(ReadXlsxFormatArgs),
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    /// Output Excel file name.
    #[arg(short = 'f', long = "file", default_value = "output.xlsx")]
    pub file_name: String,
}

#[derive(Args, Debug)]
pub struct PullFormatArgs {
    /// Whether to update the backend after formatting.
    #[arg(short = 'u', long = "update-backend", default_value_t = false)]
    pub update_backend: bool,
    /// Output Excel file name after formatting (optional).
    #[arg(short = 'o', long = "output", default_value = "formatted_output.xlsx")]
    pub output_file: String,
}

#[derive(Args, Debug)]
pub struct ReadXlsxFormatArgs {
    /// Path to the Excel file to read stops from.
    #[arg(short = 'f', long = "file", default_value = "input.xlsx")]
    pub file_path: String,
    /// Output Excel file name after formatting (optional).
    #[arg(short = 'o', long = "output", default_value = "formatted_output.xlsx")]
    pub output_file: String,

    #[arg(short = 'u', long = "update-backend", default_value_t = false)]
    pub update_backend: bool,
}

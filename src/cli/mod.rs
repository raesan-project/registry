use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub sub_commands: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    ServeQuestions(ServeQuestions),
    GenerateDatabaseRecords(GenerateDatabaseRecords),
}

#[derive(Args, Debug, Clone)]
#[command(about = "start a server to render the questions")]
pub struct ServeQuestions {
    #[arg(long, help = "path location of questions folder")]
    pub questions_folder: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "generate SQLite database records from registry")]
pub struct GenerateDatabaseRecords {
    #[arg(long, help = "path location of database")]
    pub database: String,
    #[arg(long, help = "path location of registry")]
    pub registry: String,
}

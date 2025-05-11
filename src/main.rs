use clap::Parser;
use color_eyre::eyre::{self, WrapErr};
use raesan_registry::{cli, error, registry, server};
use tokio;
use tracing_subscriber;

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = cli::CLIArgs::parse();
    match &args.sub_commands {
        cli::SubCommands::ServeQuestions(server_data) => {
            server::serve()
                .server_data(server_data.clone())
                .call()
                .await
                .wrap_err("failed to start the server")?;
        }
        cli::SubCommands::GenerateDatabaseRecords(gen_data) => {
            registry::generate_database_records()
                .gen_data(gen_data.clone())
                .call()
                .wrap_err("failed to generate database records")?;
        }
    }

    return Ok(());
}

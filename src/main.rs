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
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = cli::CLIArgs::parse();
    match &args.sub_commands {
        cli::SubCommands::ServeQuestions(serve_data) => {
            server::serve()
                .serve_data(serve_data.clone())
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

use clap::Parser;
use color_eyre::eyre;
use raesan_registry::{cli, registry, server};
use tokio;

#[tokio::main]
async fn main() -> eyre::Result<()> {
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
                .await?;
        }
        cli::SubCommands::GenerateDatabaseRecords(gen_data) => {
            registry::generate_database_records()
                .gen_data(gen_data.clone())
                .call()?;
        }
    }

    return Ok(());
}

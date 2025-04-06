use clap::Parser;
use raesan_registry::{cli, registry, server};
use tokio;

#[tokio::main]
async fn main() {
    let args = cli::CLIArgs::parse();
    match &args.sub_commands {
        cli::SubCommands::ServeQuestions(serve_data) => {
            match server::serve().serve_data(serve_data.clone()).call().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to start the server, Error: {:#?}", e.to_string())
                }
            };
        }
        cli::SubCommands::GenerateDatabaseRecords(gen_data) => {
            match registry::generate_database_records()
                .gen_data(gen_data.clone())
                .call()
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Failed to generate database records, Error: {:#?}",
                        e.to_string()
                    )
                }
            }
        }
    }
}

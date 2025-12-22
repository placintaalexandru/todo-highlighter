use clap::Parser;
use language_server::infrastructure::runtime;
use tower_lsp_server::{LspService, Server};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "TODO Highlight Language Server CLI", long_about = None)]
struct Args {}

#[tokio::main]
async fn main() {
    let _ = Args::parse();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(runtime::init_server);
    Server::new(stdin, stdout, socket).serve(service).await;
}

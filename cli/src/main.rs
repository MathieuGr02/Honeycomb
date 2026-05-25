use std::io::Bytes;

use clap::Parser;
use honeycomb_cli::{
    cli::{CLI, CLICommands},
    logger::init_logger,
    traits::ToJsonBody,
};
use hyper::{Method, Request};
use hyper_util::client::legacy::Client;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use log::{debug, error, info};
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::TRACE);
    let args = CLI::parse();

    match args.command {
        CLICommands::Run(run_args) => {
            let uri: hyper::Uri =
                Uri::new("./honeycomb.socket", "localhost/containers/create").into();

            let client = Client::unix();
            let body = run_args.to_json_body().unwrap();
            info!("{}", body);
            let request = Request::builder()
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .header("Content-Length", body.len())
                .uri(uri)
                .body(body)
                .unwrap();

            let response = client.request(request).await.unwrap();
            info!("{}", response.status());
        }
        CLICommands::Build(build_args) => {}
    }
}

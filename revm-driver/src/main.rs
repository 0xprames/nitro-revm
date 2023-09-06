use alloy_primitives::{Address, U256};
use clap::{App, AppSettings, Arg, SubCommand};

use revm_driver::client;
use revm_driver::command_parser::ClientArgs;
use revm_driver::create_app;

// This payload should be generalized to include all the pre-state for each
// simulation.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Payload {
    sender: Address,
    amount: U256,
}
#[tokio::main]
async fn main() {
    let app = create_app!();
    let args = app.get_matches();

    match args.subcommand() {
        Some(("client", args)) => {
            let client_args = ClientArgs::new_with(args).unwrap();
            client(client_args).await.unwrap();
        }
        Some(_) | None => (),
    }
}

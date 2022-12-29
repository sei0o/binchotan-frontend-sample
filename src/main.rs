use clap::{Parser, Subcommand};
use colored_json::ToColoredJson;
use serde_json::json;
use std::{
    env,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use anyhow::{Context, Result};

const JSONRPC_VERSION: &str = "2.0";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Calls status method
    Status,

    /// Calls plain_request method
    Plain {
        http_method: String,
        endpoint: String,
        params: String,
    },

    /// Calls home_timeline method
    HomeTimeline {
        params: Option<String>, // in JSON format
    },

    /// Calls account.list method to list authorized accounts
    AccountList { session: String },

    /// Calls account.add method to authenticate with another account
    AccountAdd { session: Option<String> },
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let args = Args::parse();

    let sock_path = env::var("SOCKET_PATH")?;
    let mut stream = UnixStream::connect(sock_path)
        .context("could not connect to the socket. Is the backend running?")?;
    let id = uuid::Uuid::new_v4().to_string();

    let mut add_account = false;
    let payload = match args.command {
        Commands::Status => {
            println!("sending status request");

            json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "method": "v0.status",
                // FIXME: this should be able to be omitted (see backend's Method struct)
                "params": {},
            })
            .to_string()
        }
        Commands::Plain {
            http_method,
            endpoint,
            params,
        } => {
            println!("sending plain request");
            let params_val: serde_json::Value = serde_json::from_str(&params)?;

            json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "params": {
                    "http_method": http_method,
                    "endpoint": endpoint,
                    // parse here
                    "api_params": params_val,
                },
                "method": "v0.plain",
            })
            .to_string()
        }
        Commands::HomeTimeline { params } => {
            println!("sending home_timeline request");
            let params = params.unwrap_or_else(|| "{}".to_string());
            let params: serde_json::Value = serde_json::from_str(&params)?;

            json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "params": params,
                "method": "v0.home_timeline",
            })
            .to_string()
        }
        Commands::AccountList { session } => {
            println!("sending accounts list request");

            json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "params": {
                    "session_key": session,
                },
                "method": "v0.account.list",
            })
            .to_string()
        }
        Commands::AccountAdd { session } => {
            println!("sending accounts add request");
            add_account = true;

            json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "params": {
                    "session_key": session
                },
                "method": "v0.account.add",
            })
            .to_string()
        }
    };

    println!("{payload}");

    let mut resp = String::new();
    stream.write_all(payload.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    stream.read_to_string(&mut resp)?;

    println!("{}", resp.to_colored_json_auto()?);

    if add_account {
        let payload: serde_json::Value = serde_json::from_str(&resp)?;
        let auth_url = payload["result"]["auth_url"]
            .as_str()
            .expect("auth_url is not given");

        open::that(auth_url).unwrap_or_else(|_| println!("Browse to: {}", auth_url));
    }

    Ok(())
}

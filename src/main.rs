use clap::{Parser, Subcommand};
use colored_json::ToColoredJson;
use serde_json::json;
use std::{
    collections::HashMap,
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
        // TODO
    },
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let sock_path = env::var("SOCKET_PATH")?;
    let mut stream = UnixStream::connect(sock_path)
        .context("could not connect to the socket. Is the backend running?")?;
    let id = uuid::Uuid::new_v4().to_string();

    match args.command {
        Commands::Status => {
            println!("sending status request");

            let payload = json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "method": "v0.status",
            })
            .to_string();
            println!("{payload}");
            let mut resp = String::new();
            stream.write_all(payload.as_bytes())?;
            stream.write_all(b"\n")?;
            stream.flush()?;
            stream.read_to_string(&mut resp)?;

            println!("{}", resp.to_colored_json_auto()?);
        }
        Commands::Plain {
            http_method,
            endpoint,
            params,
        } => {
            println!("sending plain request");
            let params_val: serde_json::Value = serde_json::from_str(&params)?;

            let payload = json!({
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
            .to_string();
            let mut resp = String::new();
            stream.write_all(payload.as_bytes())?;
            stream.write_all(b"\n")?;
            stream.flush()?;
            stream.read_to_string(&mut resp)?;

            println!("{}", resp.to_colored_json_auto()?);
        }
        Commands::HomeTimeline {} => {
            println!("sending home_timeline request");

            let payload = json!({
                "jsonrpc": JSONRPC_VERSION,
                "id": id,
                "method": "v0.home_timeline",
            })
            .to_string();
            let mut resp = String::new();
            stream.write_all(payload.as_bytes())?;
            stream.write_all(b"\n")?;
            stream.flush()?;
            stream.read_to_string(&mut resp)?;

            println!("{}", resp.to_colored_json_auto()?);
        }
    }

    Ok(())
}

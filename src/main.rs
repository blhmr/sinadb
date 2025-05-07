use std::{env, error::Error};
use server::start_server;

mod server;
mod database;
mod response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let host = match args.last() {
        Some(host) => host,
        None => {
            return Err("Usage: sinadb <host:port>".into());
        },
    };
    start_server(host).await?;
    Ok(())
}
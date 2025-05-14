use std::error::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use crate::{database::Database, response::Response};

pub async fn start_server(host: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(host).await?;
    println!("Server is listening on {}...", host);
    let database = Database::new();
    loop {
        let (conn, addr) = listener.accept().await?;
        println!("[+] New connection from: {}", addr);
        let database = database.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(conn, &database).await {
                eprintln!("[X] Error occured while handling client: {}", e);
            }
        });
    }
}

pub async fn handle_client(mut conn: TcpStream, database: &Database) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 1024];
    let n = conn.read(&mut buffer).await?;
    if n == 0 {
        conn.write_all("No data provided".as_bytes()).await?;
    }

    let request = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    println!("[+] Got request: {}", request);
    // database.print_all().await;
    let response = Response::from(request);
    if let Response::Ok(method, key, value, ttl) = response {
        match method.as_str() {
            "SET" => {
                database.set(key.clone(), value.unwrap().clone(), ttl).await;
                conn.write("OK\n".as_bytes()).await?;
            },
            "GET" => {
                if key == "*" {
                    if let Some(values) = database.get_all().await {
                        let mut response = String::new();
                        for (key, value) in values.iter() {
                            response.push_str(&format!("{} = {}\n", key, value));
                        }
                        conn.write(response.as_bytes()).await?;
                    } else {
                        conn.write("none\n".as_bytes()).await?;
                    }
                } else {
                    if let Some(result) = database.get(&key).await {
                        conn.write((result + "\n").as_bytes()).await?;
                    } else {
                        conn.write("Key does not exist\n".as_bytes()).await?;
                    }
                }
            }
            "DEL" => {
                database.delete(&key).await;
                conn.write("OK\n".as_bytes()).await?;
            },
            // "GETALL" => {
            //     if let Some(values) = database.get_all().await {
            //         let mut response = String::new();
            //         for (key, value) in values.iter() {
            //             response.push_str(&format!("{} = {}\n", key, value));
            //         }
            //         conn.write(response.as_bytes()).await?;
            //     } else {
            //         conn.write("Hashmap is empty".as_bytes()).await?;
            //     }
            // },
            "GETSW" => {
                let values = database.get_sw(&key).await;
                if let Some(values) = values {
                    let mut response = String::new();
                    for (key, value) in values.iter() {
                        response.push_str(&format!("{} = {}\n", key, value));
                    }
                    conn.write(response.as_bytes()).await?;
                }
                else {
                    conn.write("No key starts with that pattern\n".as_bytes()).await?;
                }
            },
            "GETEW" => {
                let values = database.get_ew(&key).await;
                if let Some(values) = values {
                    let mut response = String::new();
                    for (key, value) in values.iter() {
                        response.push_str(&format!("{} = {}\n", key, value));
                    }
                    conn.write(response.as_bytes()).await?;
                }
                else {
                    conn.write("No key ends with that pattern\n".as_bytes()).await?;
                }
            },
            _ => (),
        }
    } else if let Response::Err(e) = response {
        conn.write((e + "\n").as_bytes()).await?;
    }

    Ok(())
}
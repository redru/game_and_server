mod commands;

use crate::commands::{get_port, get_time, get_uuid, parse_command, Command};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, io};
use tokio::net::UdpSocket;
use tokio::signal;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_address = env::var("server_address").unwrap_or(String::from("0.0.0.0"));
    let server_port = env::var("server_port").unwrap_or(String::from("34254"));

    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel();

    listen_shutdown(shutdown_send);

    let socket = Arc::new(UdpSocket::bind(format!("{}:{}", server_address, server_port)).await?);
    let clients = Arc::new(Mutex::new(HashMap::new()));

    'server_loop: loop {
        let s = socket.clone();
        let mut buf = [0; 1024];
        let clients = clients.clone();

        tokio::select! {
            // Break if shutdown signal was received
            _ = shutdown_recv.recv() => break 'server_loop,
            // Timeout callback
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {},
            // Datagram received callback
            recv_result = s.recv_from(&mut buf) => {
                let (amt, address) = match recv_result {
                    Ok(val) => val,
                    Err(err) => {
                        eprintln!("error reading the datagram: {}", err);
                        continue;
                    },
                };

                // Handle the datagram in another thread
                tokio::spawn(async move {
                    handle_datagram_received(Context::from((address, buf, amt)), s, clients).await;
                });
            }
        }
    }

    Ok(())
}

fn listen_shutdown(shutdown_send: UnboundedSender<()>) {
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                shutdown_send
                    .send(())
                    .expect("failed to send shutdown signal");
            }
            Err(err) => {
                eprintln!("unable to listen for shutdown signal: {}", err);
                shutdown_send
                    .send(())
                    .expect("failed to send shutdown signal");
            }
        }
    });
}

struct Context {
    address: SocketAddr,
    data: Vec<u8>, // Allocate once and always extract references from it
}

impl From<(SocketAddr, [u8; 1024], usize)> for Context {
    fn from(value: (SocketAddr, [u8; 1024], usize)) -> Self {
        Self {
            address: value.0,
            data: Vec::from(&value.1[..value.2]),
        }
    }
}

struct Client {
    address: SocketAddr,
}

async fn handle_datagram_received(
    context: Context,
    socket: Arc<UdpSocket>,
    clients: Arc<Mutex<HashMap<String, Client>>>,
) {
    println!(
        "received datagram from {:}: {:}",
        context.address,
        String::from_utf8_lossy(&context.data)
    );

    let command = parse_command(&context.data);

    if let Some(command) = command {
        match command {
            Command::Handshake => handle_handshake(&context, &clients).await,
            Command::TotalTime => handle_total_time(&context, &socket, &clients).await,
        }
    } else {
        println!("unknown command {:?}", command);
        return;
    }
}

async fn handle_handshake(context: &Context, clients: &Arc<Mutex<HashMap<String, Client>>>) {
    let uuid = get_uuid(&context.data);
    let port = get_port(&context.data);

    let client = Client {
        address: SocketAddr::new(context.address.ip(), port),
    };

    clients.lock().await.insert(uuid.to_string(), client);
}

async fn handle_total_time(
    context: &Context,
    socket: &Arc<UdpSocket>,
    clients: &Arc<Mutex<HashMap<String, Client>>>,
) {
    let uuid = get_uuid(&context.data);
    let time = get_time(&context.data);

    let clients = clients.lock().await;
    let client_address = clients.get(uuid).unwrap().address;

    socket
        .send_to(time, client_address)
        .await
        .expect("send failed");
}

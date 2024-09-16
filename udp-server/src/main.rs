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
                    handle_datagram_received(s, amt, address, buf, clients).await;
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

struct Client {
    address: SocketAddr,
}

async fn handle_datagram_received(
    socket: Arc<UdpSocket>,
    amt: usize,
    address: SocketAddr,
    buf: [u8; 1024],
    clients: Arc<Mutex<HashMap<String, Client>>>,
) {
    println!(
        "received datagram from {:}: {:}",
        address,
        String::from_utf8_lossy(&buf[..])
    );

    let command = String::from_utf8_lossy(&buf[0..2]);

    match command.as_ref() {
        "00" => {
            let uuid = String::from_utf8_lossy(&buf[2..38]).to_string();
            let port = String::from_utf8_lossy(&buf[38..43]).to_string();

            let client = Client {
                address: SocketAddr::new(address.ip(), port.parse().unwrap()),
            };

            clients.lock().await.insert(uuid, client);
        }
        "01" => {
            let uuid = std::str::from_utf8(&buf[2..38]).unwrap();
            let data = &buf[38..amt];

            let clients = clients.lock().await;
            let client_address = clients.get(uuid).unwrap().address;

            socket
                .send_to(data, client_address)
                .await
                .expect("send failed");
        }
        _ => {
            println!("unknown command {:}", command);
            return;
        }
    }
}

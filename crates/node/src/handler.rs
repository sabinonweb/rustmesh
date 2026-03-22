use core_mesh::identity::{Identity, Peer};
use quinn::Connection;

// receive message
pub async fn handle_incoming_connection(connection: Connection, my_id: Identity) {
    println!("Keep connection alive!");

    loop {
        match connection.accept_bi().await {
            Ok((mut send, mut recv)) => {
                let mut buf = vec![0; 1024];

                match recv.read(&mut buf).await {
                    Ok(Some(n)) => {
                        let message = String::from_utf8_lossy(&buf[..n]);
                        println!("Message from client: {}", message);

                        let reply = format!("{} says hello to you!", my_id.encode());
                        let _ = send.write_all(reply.as_bytes()).await;
                        let _ = send.finish();
                    }
                    Ok(None) => {
                        println!("Client closed stream");
                        break;
                    }
                    Err(e) => {
                        println!("Read error: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Connection closed: {}", e);
                break;
            }
        }
    }

    println!("Connection handler ended");
}

// send message
pub async fn handle_outgoing_connection(connection: Connection, peer: Peer) -> anyhow::Result<()> {
    println!("Inside the handler");

    let (mut send, mut recv) = connection.open_bi().await?;
    let message = format!("Hello from client {:?}", peer.id);

    println!("Checkpoint 1");
    send.write_all(message.as_bytes()).await?;
    println!("Checkpoint 2");

    println!("Checkpoint 3");
    let mut buf = vec![0; 1024];
    println!("{:?}", buf);
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 1024];
            loop {
                match recv.read(&mut buf).await {
                    Ok(Some(n)) => {
                        let msg = String::from_utf8_lossy(&buf[..n]);
                        println!("Received: {}", msg);
                    }
                    Ok(None) => {
                        println!("Stream closed by peer");
                        break;
                    }
                    Err(e) => {
                        println!("Read error: {}", e);
                        break;
                    }
                }
            }
        }
    });
    println!("Ran the handler");

    Ok(())
}

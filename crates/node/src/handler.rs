use core::identity::{Identity, Peer};
use quinn::Connection;

pub async fn receive_handle_connection(connection: Connection, my_id: Identity) {
    if let Ok((mut send, mut recv)) = connection.accept_bi().await {
        let mut buf = vec![0; 1024];
        if let Ok(n) = recv.read(&mut buf).await {
            let n = n.unwrap();
            let message = String::from_utf8_lossy(&buf[..n]);
            println!("Message from client: {:?}", message);

            let reply = format!("{} says hello to you!", my_id.peer_id());
            let _ = send.write_all(reply.as_bytes()).await;

            let mut buf = vec![0; 1024];
            let n = recv.read(&mut buf).await.unwrap().unwrap();
            let reply = String::from_utf8_lossy(&buf[..n]);
            println!("Reply: {:?}", reply);
        }
    }
}

pub async fn send_handle_connection(connection: Connection, peer: Peer) -> anyhow::Result<()> {
    let (mut send, mut recv) = connection.open_bi().await?;
    let message = format!("Hello from client {}", peer.id.peer_id());

    send.write_all(message.as_bytes()).await?;

    let mut buf = vec![0; 1024];
    match recv.read(&mut buf).await {
        Ok(Some(n)) => {
            let reply = String::from_utf8_lossy(&buf[..n]);
            println!("Reply: {:?}", reply);
        }
        Ok(None) => {
            println!("Stream closed by server");
        }
        Err(e) => {
            println!("Error reading from stream: {}", e);
        }
    }

    Ok(())
}

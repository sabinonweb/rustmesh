use libp2p::futures::StreamExt;
use libp2p::{gossipsub::IdentTopic, Swarm};
use rustmesh_core::behaviour::RustMeshBehaviour;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

use crate::handler::handle_events;

pub async fn event_loop(swarm: &mut Swarm<RustMeshBehaviour>, node_name: &str) {
    let mut publish_interval = tokio::time::interval(Duration::from_secs(2));

    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
               handle_events(node_name, event, swarm).await;
           }

           _ = sleep(Duration::from_secs(30)) => {
               info!("[{}] Heartbeat from {}", node_name, node_name);
           },


           _ = publish_interval.tick() => {
                let topic = IdentTopic::new("mesh/messages");
                let message = format!("Hello from [{}]", node_name);
                match swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes().to_vec()) {
                     Ok(_) => info!("[{}] Published: {}", node_name, message),
                    Err(e) => error!("[{}] Publish failed: {}", node_name, e),
                 }
           }
        }
    }
}

use futures::future::select;
use futures::StreamExt;
use libp2p::{
    gossipsub, mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    SwarmBuilder,
};
use std::{
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
};

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_quic()
        .with_behaviour(|key| {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .build();

    let topic = gossipsub::IdentTopic::new("test-net");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;

    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    loop {
        select! {
                Ok(Some(line)) = stdin.next_line() => {
                    if let Err(e) = swarm
                        .behaviour_mut().gossipsub
                        .publish(topic.clone(), line.as_bytes()) {
                        println!("Publish error: {e:?}");
                    }
                }

                event = swarm.select_next_some() => match event {
                   SwarmEvent::Behaviour(event) => match event {
                       MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers)) => {
                           for (peer_id, _multiaddr) in peers {
                               println!("mDNS has discovered a new peer with id: {:?}", peer_id);
                               swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                           }
                       }
                       MyBehaviourEvent::Mdns(mdns::Event::Expired(peers)) => {
                           for (peer_id, _multiaddr) in peers {
                               println!("mDNS peer has expired: {peer_id}");
                           }
                       }
                        MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message
                        }) => {
                            println!(
                    "Got message: '{}' with id: {id} from peer: {peer_id}",
                    String::from_utf8_lossy(&message.data)
                );
                        }
                        _ => {}
                    }
                   SwarmEvent::NewListenAddr { address, .. } => {
            println!("Local node is listening on {address}");
        },

                       _ => {}
                }
            }
    }
}

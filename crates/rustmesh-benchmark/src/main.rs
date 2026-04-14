use futures::stream::StreamExt;
use libp2p::{
    gossipsub::{self, IdentTopic},
    identity,
    swarm::{self, dial_opts, SwarmEvent},
    Multiaddr, PeerId, SwarmBuilder,
};
use rustmesh_core::{
    behaviour::{RustMeshBehaviour, RustMeshEvent},
    config::NodeConfig,
    error::RustMeshError,
    init_tracing, Result,
};
use rustmesh_node::event::{event_loop, get_swarm};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing("debug");

    let (mut swarm1, peer_id1) = get_swarm("bench1");
    let (mut swarm2, peer_id2) = get_swarm("bench2");

    let _ = swarm1.listen_on("/ip4/192.168.110.7/udp/9001/quic-v1".parse().unwrap());
    let _ = swarm1.listen_on("/ip4/192.168.110.7/udp/9002/quic-v1".parse().unwrap());

    let _ = event_loop(&mut swarm1, "bench1");

    tokio::spawn(async move {
        let _ = event_loop(&mut swarm2, "bench2");
    });

    Ok(())
}

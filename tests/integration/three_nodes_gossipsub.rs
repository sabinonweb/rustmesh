use crate::create_test_nodes;
use libp2p::Multiaddr;
use rustmesh_core::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_three_nodes() {
    let mut nodes = create_test_nodes(3).await;
    init_tracing("debug");

    for (i, (swarm, _)) in nodes.iter_mut().enumerate() {
        let multiaddr = format!("/ip4/127.0.0.1/udp/{}/quic-v1", 9000 + i);
        swarm.listen_on(multiaddr.parse().unwrap()).unwrap();
    }

    let _ = tokio::time::sleep(std::time::Duration::from_secs(2));

    let mut nodes_multiaddr: Vec<Multiaddr> = Vec::new();
    for (i, (_swarm, peer_id)) in nodes.into_iter().enumerate() {
        let id = format!("/ip4/127.0.0.1/udp/{}/quic-v1/p2p/{}", 9000 + i, peer_id);
        nodes_multiaddr.push(id.parse().unwrap());
    }
}

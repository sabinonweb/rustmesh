use libp2p::{gossipsub, identify, kad, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "RustMeshEvent")]
pub struct RustMeshBehaviour {
    pub gossipsub: gossipsub::Behaviour,

    pub identify: identify::Behaviour,

    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

pub enum RustMeshEvent {
    Gossipsub(gossipsub::Event),
    Identify(identify::Event),
    Kademlia(kad::Event),
}

impl From<gossipsub::Event> for RustMeshEvent {
    fn from(event: gossipsub::Event) -> Self {
        RustMeshEvent::Gossipsub(event)
    }
}

impl From<identify::Event> for RustMeshEvent {
    fn from(event: identify::Event) -> Self {
        RustMeshEvent::Identify(event)
    }
}

impl From<kad::Event> for RustMeshEvent {
    fn from(event: kad::Event) -> Self {
        RustMeshEvent::Kademlia(event)
    }
}

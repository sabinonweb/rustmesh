use libp2p::{gossipsub, identify, kad, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "RustMeshEvent")]
pub struct RustMeshBehaviour {
    pub gossipsub: gossipsub::Behaviour,

    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,

    pub identity: identify::Behaviour,
}

#[derive(Debug)]
pub enum RustMeshEvent {
    Gossipsub(gossipsub::Event),
    Kademlia(kad::Event),
    Identify(identify::Event),
}

impl From<gossipsub::Event> for RustMeshEvent {
    fn from(value: gossipsub::Event) -> Self {
        RustMeshEvent::Gossipsub(value)
    }
}

impl From<kad::Event> for RustMeshEvent {
    fn from(value: kad::Event) -> Self {
        RustMeshEvent::Kademlia(value)
    }
}

impl From<identify::Event> for RustMeshEvent {
    fn from(value: identify::Event) -> Self {
        RustMeshEvent::Identify(value)
    }
}

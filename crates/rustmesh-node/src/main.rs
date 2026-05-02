use clap::Parser;
use rustmesh_core::{
    init_tracing,
    transport::{quic::QuicTransport, Transport},
    Result,
};
use rustmesh_node::event::event_loop;
use tracing::info;

#[derive(Parser, Debug, Clone)]
#[command(name = "RustMesh Node")]
#[command(about = "Peer-to-peer mesh network node", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "rustmesh-node")]
    name: String,

    #[arg(short, long, default_value = "/ip4/0.0.0.0/udp/0/quic-v1")]
    listen: String,

    #[arg(short = 'g', long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_tracing(&args.log_level);
    let mut transport = QuicTransport::new(&args.name);
    transport.listen(&args.listen)?;

    info!("[{}] Listening on {}", args.name, &args.listen);

    event_loop(&mut transport.swarm, &args.name).await;

    Ok(())
}

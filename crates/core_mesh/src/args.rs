use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser, Debug, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub ip: String,

    #[arg(short, long)]
    pub port: u16,
}

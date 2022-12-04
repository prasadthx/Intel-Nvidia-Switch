use crate::graphics::GraphicsMode;
use clap::{Parser, ColorChoice};

#[derive(Parser, Debug)]
#[clap(color=ColorChoice::Always)]
#[command(color=ColorChoice::Always, about,version)]
pub struct Args {
    /// Choose the Graphics Mode: Nvidia or Intel or Hybrid.
    #[arg(long)]
    #[arg(short='S')]
    pub switch: Option<GraphicsMode>,
    
    /// Run any program with Nvidia GPU in Hybrid Mode
    #[arg(long)]
    pub run: Option<String>,
}


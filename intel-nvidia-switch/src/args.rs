use crate::graphics::GraphicsMode;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about,version)]
pub struct Args {
    /// Choose the Graphics Mode: Nvidia or Intel or Hybrid.
    #[arg(long)]
    #[arg(short='S')]
    pub switch: GraphicsMode,
}


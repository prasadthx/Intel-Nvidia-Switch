use crate::graphics::GraphicsMode;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Choose the Graphics Mode: Nvidia or Intel or Hybrid.
    #[arg(long)]
    pub switch: GraphicsMode,
}


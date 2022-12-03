use clap::Parser;
mod graphics;
use graphics::GraphicsMode;
mod args;

use args::Args;

fn main() {
    let args = Args::parse();
    match args.switch{
        GraphicsMode::Nvidia => {println!("Nvidia");},
        _ => {println!("{:?}", args);    },
    }
}

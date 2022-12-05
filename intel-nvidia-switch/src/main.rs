use clap::Parser;
use args::Args;

mod graphics;
mod run_on_nvidia;
mod args;
mod switch;
use graphics::GraphicsMode;
use switch::{switch_to_intel, switch_to_hybrid, switch_to_nvidia, offload_to_nvidia};

fn main() {
    let args = Args::parse();
    if let Some(mode) = args.switch {
        if mode == GraphicsMode::Nvidia {
            switch_to_nvidia()
        }
        
        if mode == GraphicsMode::Intel {
            switch_to_intel();
        }
        
        if mode == GraphicsMode::Hybrid {
            switch_to_hybrid();   
        }
    }
    
    if let Some(program) = args.run {
        offload_to_nvidia(program);
    }
}


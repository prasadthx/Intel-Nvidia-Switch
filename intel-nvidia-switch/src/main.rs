use std::env;

use clap::Parser;
mod graphics;
mod run_on_nvidia;
mod args;
mod switch;
use graphics::GraphicsMode;
use switch::switch_intel;
use run_on_nvidia::run_on_nvidia;


use args::Args;

fn main() {
    let args = Args::parse();
    if let Some(mode) = args.switch {
        if mode == GraphicsMode::Nvidia {
            
        }
        
        if mode == GraphicsMode::Intel {
            switch_to_intel()
        }
        
        if mode == GraphicsMode::Hybrid {
            
        }
    }
    
    if let Some(program) = args.run {
        run_on_nvidia(program);
    }
}

fn switch_to_intel(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Intel");
        return;
    }
    println!("Switching to Intel.....");
    switch_intel();
}

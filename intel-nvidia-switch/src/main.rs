use std::env;

use clap::Parser;
mod graphics;
mod run_on_nvidia;
mod args;
mod switch;
use graphics::GraphicsMode;
use switch::{switch_intel, switch_hybrid};
use run_on_nvidia::run_on_nvidia;


use args::Args;


fn main() {
    let args = Args::parse();
    if let Some(mode) = args.switch {
        if mode == GraphicsMode::Nvidia {
            
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

fn switch_to_intel(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Intel");
        return;
    }
    println!("Switching to Intel.....");
    switch_intel();
}

fn switch_to_hybrid(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "hybrid" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Hybrid");
        return;
    }
    
    switch_hybrid();
}

fn offload_to_nvidia(program: String) {
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[96m{}\x1b[0m", "GPU set to Intel. Change to Nvidia Mode / Hybrid Mode");
        return;
    }
    println!("Running on Nvidia.....");
    run_on_nvidia(program);
}
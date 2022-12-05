mod to_intel;
mod to_nvidia;
mod to_hybrid;

use to_intel::switch_intel;
use to_nvidia::switch_nvidia;
use to_hybrid::switch_hybrid;
use std::env;
use crate::run_on_nvidia::run_on_nvidia;

pub fn switch_to_intel(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Intel");
        return;
    }
    
    switch_intel();
}

pub fn switch_to_hybrid(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "hybrid" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Hybrid");
        return;
    }
    
    switch_hybrid();
}

pub fn switch_to_nvidia(){
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "nvidia" {
        println!("\x1b[96m{}\x1b[0m", "GPU already set to Nvidia");
        return;
    }
    
    switch_nvidia();
}

pub fn offload_to_nvidia(program: String) {
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[96m{}\x1b[0m", "GPU set to Intel. Change to Nvidia Mode / Hybrid Mode");
        return;
    }
    println!("Running on Nvidia.....");
    run_on_nvidia(program);
}
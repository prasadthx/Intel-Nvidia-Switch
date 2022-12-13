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
    if let Result::Ok(x) = gpu_status {
        if x == "intel" {
            println!("\x1b[93mGPU already set to \x1b[94mIntel Mode\x1b[0m");
            return;   
        }
    }
    
    switch_intel();
}

pub fn switch_to_hybrid(){
    let gpu_status = env::var("GPU_STATUS");
    if let Result::Ok(x) = gpu_status {
        if x == "hybrid"{
            println!("\x1b[93mGPU already set to \x1b[96mHybrid\x1b[0m");
            return;   
        }
    }
    
    switch_hybrid();
}

pub fn switch_to_nvidia(){
    let gpu_status = env::var("GPU_STATUS");
    if let Result::Ok(x) = gpu_status {
        if x == "nvidia" {
            println!("\x1b[96mGPU already set to \x1b[92mNvidia\x1b[0m");
            return;
        }
        println!("\x1b[96mGPU already set to \x1b[92mNvidia\x1b[0m");
        return;
    }
    
    switch_nvidia();
}

pub fn offload_to_nvidia(program: String) {
    let gpu_status = env::var("GPU_STATUS");
    if gpu_status.unwrap() == "intel" {
        println!("\x1b[31mGPU set to \x1b[34mIntel Mode\x1b[31m. Change to \x1b[32mNvidia Mode \x1b[31m/ \x1b[36mHybrid Mode\x1b[0m");
        return;
    }
    println!("\x1b[33mRunning on \x1b[32mNvidia\x1b[33m.....\x1b[0m");
    run_on_nvidia(program);
}
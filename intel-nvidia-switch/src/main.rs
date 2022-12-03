use clap::Parser;
mod graphics;
use graphics::GraphicsMode;
mod args;
mod switch;
use switch::switch_intel;

use args::Args;

fn main() {
    let args = Args::parse();
    match args.switch{
        GraphicsMode::Nvidia => {println!("Nvidia");},
        GraphicsMode::Intel => {switch_to_intel();},
        _ => {println!("{:?}", args);    },
    }
}

fn switch_to_intel(){
    println!("Switching to Intel.....");
    switch_intel();
}

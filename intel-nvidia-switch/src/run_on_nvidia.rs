use std::fmt::Debug;
use std::io::{self, Write};
use sh_inline::bash;
// use sh_inline::*;

pub fn run_on_nvidia(program: String) {
    // let mut v: Vec<String> = program.split_whitespace().map(str::to_string).collect();
    
    // if v.len() == 0{
    //     return;
    // }
    
    // let mut command = Command::new(&v[0]);
    // v.remove(0);
    
    // command.args(v);
    
    // let output = command.output();
                     
    // if let Result::Ok(op) = &output {
    //     io::stdout().write_all(&op.stdout).unwrap();
    // }
    // else if let Result::Err(err) = output {
    //     println!("{}", err);
    // }
    
    let program_str = format!("__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia {}", program);
    
    let output = bash!(program_str);
    
    if let Result::Err(err) = output {
        println!("{}", err);
    }
}
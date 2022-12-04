use std::fmt::Debug;

// use std::process::Command;
use std::io::{self, Write};
use cmd_lib::run_cmd;
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

    
    let output = bash!(program);
    
    if let Result::Err(err) = output {
        println!("{}", err);
    }
}
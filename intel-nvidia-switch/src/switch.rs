use std::fs::copy;

pub fn switch_intel() {
    println!("Switching to Intel");
    
    let blacklist_nvidia = copy("./configs/intel/blacklist-nvidia.conf", "/home/prasad/scripts/target/blacklist-nvidia.conf") ;
    match blacklist_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Blacklisting Nvidia Drivers");
        },
        Result::Err(x) => {println!("Error in copying file: {}", x)}
    }
    
    let shutdown_nvidia = copy("./configs/intel/blacklist-nvidia.conf", "/home/prasad/scripts/target/blacklist-nvidia.conf") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Shutting Down Nvidia Devices");
        }
        Result::Err(x) => {println!("Error in copying file: {}", x)}
    }
}
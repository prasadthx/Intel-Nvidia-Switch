use std::fs::{copy, remove_file};

pub fn switch_intel() {
    println!("Switching to Intel");
    
    let blacklist_nvidia = copy("./configs/intel/blacklist-nvidia.conf", "/etc/modprobe.d/blacklist-nvidia.conf") ;
    match blacklist_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Blacklisting Nvidia Drivers");
        },
        Result::Err(x) => {println!("Error in copying file: {}", x)}
    }
    
    let shutdown_nvidia = copy("./configs/intel/00-remove-nvidia.rules", "/etc/udev/rules.d/00-remove-nvidia.rules") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Shutting Down Nvidia Devices");
        }
        Result::Err(x) => {println!("Error in copying file: {}", x)}
    }
}

pub fn switch_hybrid() {
    println!("Switching to Hybrid...");
    
    let switch = true;
    
    if let Result(remove_intel_config) = remove_file("/etc/modprobe.d/blacklist-nvidia.conf"){
        println!("Removed Nvidia modules blacklisting");
    }
    if let remove_nvidia_udev = remove_file("/etc/udev/rules.d/00-remove-nvidia.rules"){
        println!("Removed Nvidia Shutdown Udev Rules");
    }
    
    let nvidia_pm_mod = copy("./configs/hybrid/nvidia-pm.conf", "/etc/modprobe.d/nvidia-pm.conf") ;
    match blacklist_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Nvidia Dynamic Power Management");
        },
        Result::Err(x) => {println!("Error in copying file: {}", x); switch = false;}
    }
    
    let shutdown_nvidia = copy("./configs/intel/80-nvidia-pm.rules", "/etc/udev/rules.d/80-nvidia-pm.rules") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Nvidia PM");
        }
        Result::Err(x) => {println!("Error in copying file: {}", x); switch = false;}
    }
    
    if switch {
        println!("Successfully Switched to Hybrid Mode!");
        println!("Run Any Program On Nvidia GPU using --run flag.");
    }
    
}
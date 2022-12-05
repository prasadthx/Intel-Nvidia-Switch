use std::fs::{copy, remove_file, write};

pub fn switch_intel() {
    println!("Switching to Intel");
    
    let mut switch = true;
    
    if let Result::Ok(_remove_nvidia_pm) = remove_file("/etc/modprobe.d/nvidia-pm.conf"){
        println!("\x1b[1;35mRemoved Configuration for Nvidia Dynamic Power Management\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_pm_udev )= remove_file("/etc/udev/rules.d/80-nvidia-pm.rules"){
        println!("\x1b[1;35mRemoved Nvidia PM Udev Rules\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_xorg )= remove_file("/etc/X11/xorg.conf.d/10-gpu.conf"){
        println!("\x1b[1;35mRemoved Nvidia-Only Xorg Configuration\x1b[0m");
    }
    if let Result::Ok(_remove_xrandr_xinitrc )= remove_file("./configs/nvidia/xinitrc.conf"){
        println!("\x1b[1;35mRemoved Xrandr Xinitrc Configuration\x1b[0m");
    }
    
    let blacklist_nvidia = copy("./configs/intel/blacklist-nvidia.conf", "/etc/modprobe.d/blacklist-nvidia.conf") ;
    match blacklist_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Blacklisting Nvidia Drivers");
        },
        Result::Err(x) => {println!("Error in setting configuration: {}", x); switch = false;}
    }
    
    let shutdown_nvidia = copy("./configs/intel/00-remove-nvidia.rules", "/etc/udev/rules.d/00-remove-nvidia.rules") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("Successfully added Configuration for Shutting Down Nvidia Devices");
        }
        Result::Err(x) => {println!("Error in setting configuration: {}", x); switch = false;}
    }
    
    if switch {
        println!("\x1b[1;34mSuccessfully switched to Intel Mode! Reboot the system.");
    }
}

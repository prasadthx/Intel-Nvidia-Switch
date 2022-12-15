use std::fs::{copy, remove_file, write};
use std::env;

pub fn switch_intel() {
    println!("\x1b[1;33mSwitching to \x1b[94mIntel\x1b[1;33m...\x1b[0m");
    
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
    if let Result::Ok(_remove_xrandr_xinitrc )= remove_file("/etc/X11/xinit/xinitrc.d/10-gpu-xrandr.sh"){
        println!("\x1b[1;35mRemoved Xrandr Xinitrc Configuration\x1b[0m");
    }
    
    let blacklist_nvidia = copy("/etc/intel-nvidia-switch/configs/intel/blacklist-nvidia.conf", "/etc/modprobe.d/blacklist-nvidia.conf") ;
    match blacklist_nvidia {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Blacklisting Nvidia Drivers.\x1b[0m");
        },
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    let shutdown_nvidia = copy("/etc/intel-nvidia-switch/configs/intel/00-remove-nvidia.rules", "/etc/udev/rules.d/00-remove-nvidia.rules") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Shutting Down Nvidia Devices\x1b[0m");
        }
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    if switch {
        env::set_var("GPU_STATUS", "intel");
        let gpu_status = "export GPU_STATUS=intel";
        let set_gpu_status = write("/etc/profile.d/gpu_status.sh", gpu_status);
        if let Result::Err(x) = set_gpu_status{println!("\x1b[1;31mError in setting environment variable: {}\x1b[0m", x); switch = false;}
        
        if switch {
             println!("\x1b[1;33mSuccessfully Switched to \x1b[94mIntel Mode\x1b[1;33m! Reboot the system.\x1b[0m");
        }
    }
}

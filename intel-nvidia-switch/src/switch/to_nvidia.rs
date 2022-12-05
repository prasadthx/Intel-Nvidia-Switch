use std::fs::{copy, remove_file, write};
use std::env;

pub fn switch_nvidia (){
    println!("\x1b[1;33mSwitching to \x1b[92mNvidia.\x1b[1;33m..\x1b[0m");
    
    let mut switch = true;
    
    if let Result::Ok(_remove_nvidia_blacklist) = remove_file("/etc/modprobe.d/blacklist-nvidia.conf"){
        println!("\x1b[1;35mRemoved Nvidia modules blacklisting\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_udev )= remove_file("/etc/udev/rules.d/00-remove-nvidia.rules"){
        println!("\x1b[1;35mRemoved Nvidia Shutdown Udev Rules\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_pm) = remove_file("/etc/modprobe.d/nvidia-pm.conf"){
        println!("\x1b[1;35mRemoved Configuration for Nvidia Dynamic Power Management\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_pm_udev )= remove_file("/etc/udev/rules.d/80-nvidia-pm.rules"){
        println!("\x1b[1;35mRemoved Nvidia PM Udev Rules\x1b[0m");
    }
    
    let nvidia_xorg = copy("./configs/nvidia/10-gpu.conf", "/etc/X11/xorg.conf.d/10-gpu.conf") ;
    match nvidia_xorg {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Xorg Nvidia\x1b[0m");
        },
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    let xinitrc_config = "xrandr --setprovideroutputsource modesetting NVIDIA-0
                                xrandr --auto
                                xrandr --dpi 96";
    
    let xrandr_switch = write("./configs/nvidia/xinitrc.conf", xinitrc_config) ;
    match xrandr_switch {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Xinitrc\x1b[0m");
        }
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    if switch {
        env::set_var("GPU_STATUS", "nvidia");
        let gpu_status = "GPU_STATUS=nvidia
                                export GPU_STATUS";
        let set_gpu_status = write("~/scripts/gpu_status.sh", gpu_status);
        if let Result::Err(x) = set_gpu_status{println!("\x1b[1;31mError in setting environment variable: {}\x1b[0m", x); switch = false;}
        
        if switch {
             println!("\x1b[1;33mSuccessfully Switched to \x1b[92mNvidia Mode\x1b[1;33m! Reboot the system.\x1b[0m");
        }
    }
}
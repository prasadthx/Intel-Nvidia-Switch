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

pub fn switch_hybrid() {
    println!("\x1b[1;35mSwitching to Hybrid...\x1b[0m");
    
    let mut switch = true;
    
    if let Result::Ok(_remove_nvidia_blacklist) = remove_file("/etc/modprobe.d/blacklist-nvidia.conf"){
        println!("\x1b[1;35mRemoved Nvidia modules blacklisting\x1b[0m");
    }
    if let Result::Ok(_remove_nvidia_udev )= remove_file("/etc/udev/rules.d/00-remove-nvidia.rules"){
        println!("\x1b[1;35mRemoved Nvidia Shutdown Udev Rules\x1b[0m");
    }
        if let Result::Ok(_remove_nvidia_xorg )= remove_file("/etc/X11/xorg.conf.d/10-gpu.conf"){
        println!("\x1b[1;35mRemoved Nvidia-Only Xorg Configuration\x1b[0m");
    }
    if let Result::Ok(_remove_xrandr_xinitrc )= remove_file("./configs/nvidia/xinitrc.conf"){
        println!("\x1b[1;35mRemoved Xrandr Xinitrc Configuration\x1b[0m");
    }
    
    let nvidia_pm_mod = copy("./configs/hybrid/nvidia-pm.conf", "/etc/modprobe.d/nvidia-pm.conf") ;
    match nvidia_pm_mod {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Nvidia Dynamic Power Management\x1b[0m");
        },
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    let shutdown_nvidia = copy("./configs/hybrid/80-nvidia-pm.rules", "/etc/udev/rules.d/80-nvidia-pm.rules") ;
    match shutdown_nvidia {
        Result::Ok(_x) => {
            println!("\x1b[1;35mSuccessfully added Configuration for Nvidia PM\x1b[0m");
        }
        Result::Err(x) => {println!("\x1b[1;31mError in setting configuration: {}\x1b[0m", x); switch = false;}
    }
    
    if switch {
        println!("\x1b[1;34mSuccessfully Switched to Hybrid Mode! Reboot the system.\x1b[0m");
        println!("\x1b[1;34mRun Any Program On Nvidia GPU using --run flag.\x1b[0m");
    }
}

pub fn switch_nvidia (){
    println!("\x1b[1;35mSwitching to Hybrid...\x1b[0m");
    
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
        println!("\x1b[1;3mSuccessfully Switched to Nvidia Mode! Reboot the system.\x1b[0m");
    }
}
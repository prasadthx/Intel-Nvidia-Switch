use sh_inline::bash;

pub fn run_on_nvidia(program: String) {
    
    let program_str = format!("__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia {}", program);
    
    let output = bash!(program_str);
    
    if let Result::Err(err) = output {
        println!("{}", err);
    }
}
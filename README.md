# Intel-Nvidia-Switch
A small utility to switch graphics on Intel-Nvidia Hybrid graphics devices on Linux

## It Provides:-
1. Switch GPU to Intel Only, switching off Nvidia.
2. Switch GPU to Hybrid, Intel and Nvidia both.
3. Switch GPU to Nvidia Only.
4. Run Programs on Nvidia GPU in Hybrid mode.

## Installation

Install The Proprietary NVIDIA drivers according to the linux kernel installed(nvidia/nvidia-lts). 

### For Arch Linux Users:-
1. Use AUR Helpers. eg:-
  ````shell
  paru -S intel-nvidia-switch-bin
  ````
2. Using PKGBUILD provided in source code(From arch-linux-pkg directory).
  ````shell
  makepkg -si
  ````

### For other Distros:-
Get the release or download the source code and run:-
  ````shell
  sudo ./install.sh
  ````

## Usage:-

````bash
Usage: intel-nvidia-switch [OPTIONS]

Options:
  -S, --switch <SWITCH>  Choose the Graphics Mode: Nvidia or Intel or Hybrid
      --run <RUN>        Run any program with Nvidia GPU in Hybrid Mode
  -h, --help             Print help information
  -V, --version          Print version information
````

It sets a Global Environment Variable (GPU_STATUS) in the system according to the GPU mode. The default mode on installation is Hybrid Mode.

1. To run a program on Nvidia GPU in Hybrid Mode
````bash
intel-nvidia-switch --run "<program_name>"
````
  Example:
  ````bash
  [prasad@ArchLinux ~]$ echo $GPU_STATUS
  hybrid
  [prasad@ArchLinux ~]$ glxinfo | grep "OpenGL renderer"
  OpenGL renderer string: Mesa Intel(R) UHD Graphics 630 (CFL GT2)
  [prasad@ArchLinux ~]$
  [prasad@ArchLinux ~]$ intel-nvidia-switch --run "glxinfo | grep 'OpenGL renderer'"
  Running on Nvidia.....
  OpenGL renderer string: NVIDIA GeForce GTX 1650 with Max-Q Design/PCIe/SSE2
  ````
  
2. To switch to Intel Only Mode:
````
sudo intel-nvidia-switch --switch intel
````

3. To switch to Hybrid Mode:
````bash
sudo intel-nvidia-switch --switch hybrid
````

4. To switch to Nvidia Mode:
````bash
sudo intel-nvidia-switch --switch nvidia
````


## Working:-

1. https://wiki.archlinux.org/title/Hybrid_graphics#Fully_power_down_discrete_GPU
2. https://wiki.archlinux.org/title/PRIME#PRIME_render_offload
3. https://wiki.archlinux.org/title/NVIDIA_Optimus#Use_NVIDIA_graphics_only


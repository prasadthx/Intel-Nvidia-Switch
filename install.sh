#!/bin/bash

echo "Installing intel-nvidia-switch"

conf_dir="/etc/intel-nvidia-switch/configs"

install -Dm755 bin/intel-nvidia-switch "/usr/bin/intel-nvidia-switch"

install -Dm644 configs/intel/00-remove-nvidia.rules "${conf_dir}/intel/00-remove-nvidia.rules"
install -Dm644 configs/intel/blacklist-nvidia.conf "${conf_dir}/intel/blacklist-nvidia.conf"
install -Dm644 configs/nvidia/10-gpu.conf "${conf_dir}/nvidia/10-gpu.conf"

install -Dm644 configs/nvidia/10-gpu-xrandr.sh "${conf_dir}/nvidia/10-gpu-xrandr.sh"

install -Dm644 configs/hybrid/80-nvidia-pm.rules "${conf_dir}/hybrid/80-nvidia-pm.rules"

install -Dm644 configs/hybrid/nvidia-pm.conf "${conf_dir}/hybrid/nvidia-pm.conf"

echo "Succesfully added package intel-nvidia-switch"

cp /etc/intel-nvidia-switch/configs/hybrid/nvidia-pm.conf /etc/modprobe.d/nvidia-pm.conf

cp /etc/intel-nvidia-switch/configs/hybrid/80-nvidia-pm.rules /etc/udev/rules.d/80-nvidia-pm.rules

export GPU_STATUS=hybrid > /etc/profile.d/gpu_status.sh

echo -e "\e[1;33mSetting Default Setting to \e[96mHybrid Mode\e[1;33m! Reboot the system.\x1b[0m"

echo "For more information run intel-nvidia-switch --help"

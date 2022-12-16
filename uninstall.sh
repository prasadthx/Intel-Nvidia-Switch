#!/bin/bash

echo "Uninstalling intel-nvidia-switch"

rm /usr/bin/intel-nvidia-switch
rm -r /etc/intel-nvidia-switch
rm /etc/profile.d/gpu_status.sh

echo "Finished Uninstalling intel-nvidia-switch"


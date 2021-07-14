#!/bin/bash

pacman -Qi linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux
pacman -Qi linux-lts > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-lts

pkexec pacman -Syu --noconfirm koompi-linux koompi-linux-headers acpi_call-koompi-linux

exit 0;



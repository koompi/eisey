#!/bin/bash

pacman -Qi linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux
pacman -Qi linux-lts > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-lts
pacman -Qi linux-zen > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-zen

pkexec pacman -Syu --noconfirm koompi-linux koompi-linux-headers acpi_call-koompi-linux
pkexec grub-mkconfig -o /boot/grub/grub.cfg

exit 0;



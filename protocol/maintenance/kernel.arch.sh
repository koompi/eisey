#!/bin/bash

pacman -Qi koompi-linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc koompi-linux

pacman -Qi linux-lts > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-lts

pacman -Qi linux-zen > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-zen

pkexec pacman -Syu --noconfirm linux linux-headers acpi_call
pkexec grub-mkconfig -o /boot/grub/grub.cfg

exit 0;



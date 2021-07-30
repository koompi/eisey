#!/bin/bash

pacman -Qi linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux

pacman -Qi linux-lts > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-lts

pacman -Qi koompi-linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc koompi-linux

pkexec pacman -Syu --noconfirm linux-zen linux-zen-headers acpi_call
pkexec grub-mkconfig -o /boot/grub/grub.cfg

exit 0;



#!/bin/bash

pacman -Qi linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux

pacman -Qi linux-zen > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc linux-zen

pacman -Qi koompi-linux > /dev/null
[[ $? == 0 ]] && yes | pkexec pacman -Rcc koompi-linux

pkexec pacman -Syu --noconfirm linux-lts linux-lts-headers acpi_call-lts
pkexec grub-mkconfig -o /boot/grub/grub.cfg

exit 0;



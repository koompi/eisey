#!/bin/bash

pkexec grub-install --target=x86_64-efi --bootloader-id=KOOMPI_OS --recheck
pkexec grub-mkconfig -o /boot/grub/grub.cfg

exit 0;
#!/bin/bash

pacman -Qqn | pkexec pacman -S - --noconfirm --overwrite="*"

exit 0;
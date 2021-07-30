#!/bin/bash

pkexec pacman -Scc --noconfirm
rm -rf $HOME/.cache/pi
exit 0;
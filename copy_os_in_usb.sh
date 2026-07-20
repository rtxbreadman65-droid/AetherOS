#!/bin/bash

lsblk
sudo mkfs.vfat -F 32 /dev/sda1
sudo mount /dev/sda1 /mnt
sudo mkdir -p /mnt/EFI/BOOT
sudo cp target/x86_64-unknown-uefi/debug/NetworkingOS.efi /mnt/EFI/BOOT/BOOTX64.EFI
ls /mnt/EFI/BOOT
sudo umount /mnt

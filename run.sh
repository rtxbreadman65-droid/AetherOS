#!/bin/bash

rustc --target x86_64-unknown-none \
    -C opt-level=3 \
    -C panic=abort \
    -C code-model=kernel \
    -C relocation-model=static \
    -C target-feature=-sse,-sse2,-avx \
    -C link-arg=-Tlinker.ld \
    kernel.rs -o kernel.elf

objcopy -O binary kernel.elf kernel.bin

# 1. UEFI ke liye compile karega
cargo build --target x86_64-unknown-uefi

# 2. Agar build kamyab hui, toh file copy kar
if [ $? -eq 0 ]; then
    echo "⚡ Compilation successful! Copying file..."
    mkdir -p esp/EFI/BOOT
    cp target/x86_64-unknown-uefi/debug/NetworkingOS.efi esp/EFI/BOOT/BOOTX64.EFI
    
    # 3. QEMU command mein -serial stdio add kar diya hai
    # 3. QEMU command mein audio flags add kar diye hain
    echo "🖥 Launching QEMU..."
    qemu-system-x86_64 \
       -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd \
       -drive format=raw,file=fat:rw:esp,media=disk \
       -net none \
       -serial stdio \
       -audiodev alsa,id=audio0 \
       -machine pcspk-audiodev=audio0 
else
    echo "❌ Build failed! Check errors above."
fi

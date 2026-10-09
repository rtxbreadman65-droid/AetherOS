# AetherOS

**A pure Rust, from-scratch bare-metal operating system kernel for x86_64 (UEFI)**

Kernel built completely from scratch with **zero external crates**.  
Made by a 15-year-old developer learning low-level systems programming.

---

## Features

- Custom UEFI bootloader
- Framebuffer graphics rendering
- Global Descriptor Table (GDT)
- Interrupt Descriptor Table (IDT)
- PS/2 Keyboard driver
- Basic interrupt handling
- Custom print system
- Screen color manipulation
- Early multiprocessor experiments (APIC / IOAPIC)

---

## Screenshots

### Bootloader Entry
<img width="800" alt="Bootloader" src="https://github.com/user-attachments/assets/981d3477-f919-4e62-bfaa-92827df50925" />

### Kernel Logo
<img width="800" alt="Kernel Logo" src="https://github.com/user-attachments/assets/f570bfd6-d300-455a-9b47-2d6e90afb809" />

### Boot Logs
<img width="800" alt="Boot Logs" src="https://github.com/user-attachments/assets/ab90fd6b-a513-4819-aaa6-1fd6ca8dea60" />

### Terminal (Black)
<img width="800" alt="Black Terminal" src="https://github.com/user-attachments/assets/28c9d6bd-5b91-4c0a-8d7d-905db51fdec9" />

### Terminal (Blue)
<img width="800" alt="Blue Terminal" src="https://github.com/user-attachments/assets/d9f3cb3c-48ff-4aec-a16d-b6aa014da56e" />

---

## About the Project

AetherOS is a hobby operating system kernel written entirely in Rust without using any external crates.  
The goal is to understand how operating systems work at the lowest level by implementing everything from scratch.

This project is still in early development. Currently focusing on:
- Improving interrupt handling
- Multiprocessor support
- Better memory management

---

## Building & Running

### Requirements
- Rust (nightly recommended)
- `x86_64-unknown-none` target
- `x86_64-unknown-uefi` target
- QEMU
- `objcopy` (from binutils)

### Build Steps

```bash
# 1. Build the kernel
rustc --target x86_64-unknown-none \
    -C opt-level=3 \
    -C panic=abort \
    -C code-model=kernel \
    -C relocation-model=static \
    -C target-feature=-sse,-sse2,-avx \
    -C link-arg=-Tlinker.ld \
    kernel.rs -o kernel.elf

objcopy -O binary kernel.elf kernel.bin

# 2. Build the UEFI bootloader
cargo build --target x86_64-unknown-uefi

# 3. Create ESP structure
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/AetherOS.efi esp/EFI/BOOT/BOOTX64.EFI

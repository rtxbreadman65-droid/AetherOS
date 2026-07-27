# BOOT SEQUENCES:
This is my operating system boot sequences.

# BOOTLOADER ENTRY:
<img width="1920" height="1080" alt="Screenshot_20260728_022715" src="https://github.com/user-attachments/assets/981d3477-f919-4e62-bfaa-92827df50925" />

# KERNEL ENTRY LOGO:
<img width="1920" height="1080" alt="Screenshot_20260725_045147" src="https://github.com/user-attachments/assets/f570bfd6-d300-455a-9b47-2d6e90afb809" />

# BOOT LOGS:
<img width="1920" height="1080" alt="Screenshot_20260725_045206" src="https://github.com/user-attachments/assets/ab90fd6b-a513-4819-aaa6-1fd6ca8dea60" />

# BLACK TERMINAL:
<img width="1920" height="1080" alt="Screenshot_20260725_045345" src="https://github.com/user-attachments/assets/28c9d6bd-5b91-4c0a-8d7d-905db51fdec9" />

# BLUE TERMINAL:
<img width="1920" height="1080" alt="Screenshot_20260725_050736" src="https://github.com/user-attachments/assets/d9f3cb3c-48ff-4aec-a16d-b6aa014da56e" />

# DEFINITION OF MY OPERATING SYSTEM:
I made this Operating system all by myself no external crate etc. This is my 2 months project I am still working on my operating system solo.
I made this operating system with zero abstraction. I am only 15 years old. if you are new in OS Engineering I hope my OS helps you understanding 
basics of OS Engineering. My operating system only runs on intel. you can customize my kernel to run on your PC. This operating system runs perfectly 
on mine. My operating system only runs in uefi systems. To run my operating system please use my bootloader only which is in src folder, and my compilations command. If you need any help here is my instagram id: sys.arshman.

# COMPILATION COMMAND:

rustc --target x86_64-unknown-none \
    -C opt-level=3 \
    -C panic=abort \
    -C code-model=kernel \
    -C relocation-model=static \
    -C target-feature=-sse,-sse2,-avx \
    -C link-arg=-Tlinker.ld \
    kernel.rs -o kernel.elf

objcopy -O binary kernel.elf kernel.bin

cargo build --target x86_64-unknown-uefi

mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/NetworkingOS.efi esp/EFI/BOOT/BOOTX64.EFI

# AUTHER: ARSHMAN FARHAN

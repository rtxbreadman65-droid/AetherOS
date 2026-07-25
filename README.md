# BOOT SEQUENCES:

# BootLoader:
<img width="1920" height="1080" alt="Screenshot_20260725_045140" src="https://github.com/user-attachments/assets/4d6d465a-0f6e-45a9-af73-52a96dcb50fd" />
Hobby OS bootloader is just my old OS bootloader.

# Logo:
<img width="1920" height="1080" alt="Screenshot_20260725_045147" src="https://github.com/user-attachments/assets/76284f34-1a34-4807-8dba-71661e09c415" />

# BootLogs:
<img width="1920" height="1080" alt="Screenshot_20260725_045206" src="https://github.com/user-attachments/assets/e29cf04a-4065-4095-b075-958b87befdeb" />

# Black Terminal:
<img width="1920" height="1080" alt="Screenshot_20260725_045345" src="https://github.com/user-attachments/assets/04b9a1ee-2ce7-4b6d-bbba-75f8d911e801" />

# Blue Terminal:
<img width="1920" height="1080" alt="Screenshot_20260725_050736" src="https://github.com/user-attachments/assets/d3754dee-e2d0-4d2b-b376-a9d0ddcdd889" />

# ADVANTAGES OF MY OPERATING SYSTEM:
# Advantage:
If you are new in OS Engineering i hope my OS helps you alot because this OS is made by 15 years old you can use my drivers in your customs kernel but dont forget me Arshman Farhan

# DEFINITION OF MY OPERATING SYSTEM:
# AetherOS
main.rs is my bootloader please use my bootloader only. I am currently working on this OS. This OS is only runs on intel. some driver may not work on different motherboard. use my compilation command
put main.rs in src folder. unzip AetherOS_efi.zip. This is my 2 months project not that much advanced, But you can try my OS if you love bare-metal and runtime jit. this is only for uefi. Inspired my Terry Davis. use my Cargo.toml and Cargo.lock. if you need any help here is my instagram id: sys.arshman.


# COMPILATIONS COMMANDS:

rustc --target x86_64-unknown-none     -C opt-level=3     -C panic=abort     -C code-model=kernel     -C relocation-model=static     -C target-feature=-mmx,-sse,-sse2,-avx,-soft-float     -C link-arg=-Tlinker.ld     kernel.rs -o kernel.elf

objcopy -O binary kernel.elf kernel.bin

cargo build --target x86_64-unknown-uefi

mkdir -p esp/EFI/BOOT

cp target/x86_64-unknown-uefi/debug/NetworkingOS.efi esp/EFI/BOOT/BOOTX64.EFI

# Author Name: Arshman Farhan

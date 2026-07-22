# AetherOS
main.rs is my bootloader please use my bootloader only. I am currently working on this OS. This OS is only runs on intel. some driver may not work on different motherboard. use my compilation command
put main.rs in src folder. unzip AetherOS_efi.zip. This is my 2 months project not that much advanced bruh but you can try my OS if you love bare-metal and runtime jit. this is only for uefi. Inspired my Terry Davis. use my Cargo.toml and Cargo.lock. idk if this shits boot on your computer. ts runs on my system perfectly. if you need any help here is my instagram id: sys.arshman.
Don't forget my name: Arshman Farhan (world most best programmer)

rustc --target x86_64-unknown-none     -C opt-level=3     -C panic=abort     -C code-model=kernel     -C relocation-model=static     -C target-feature=-mmx,-sse,-sse2,-avx,-soft-float     -C link-arg=-Tlinker.ld     kernel.rs -o kernel.elf
objcopy -O binary kernel.elf kernel.bin

cargo build --target x86_64-unknown-uefi
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/NetworkingOS.efi esp/EFI/BOOT/BOOTX64.EFI

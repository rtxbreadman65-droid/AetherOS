[org 0x8000]
[bits 16]

section .text
global _start

_start:

        cli 
        cld

        xor dx, dx
        mov ds, dx
        mov es, dx
        mov ss, dx
        xor ax, ax

        mov byte [0x7000], 1

        lgdt [gdt_descriptor]

        mov eax, cr0
        or eax, 1
        mov cr0, eax

        jmp 0x08:pmode_32_start

align 8
gdt_start:
    dq 0x0000000000000000
    dq 0x00CF9A000000FFFF
    dq 0x00CF92000000FFFF
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

pmode_32_start:
[bits 32]
pmode_32:

        mov ax, 0x10
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov ss, ax

        mov esp, 0x7F000

        xor eax, eax
        mov eax, cr4
        or eax, 00100000b
        mov cr4, eax
        mov eax, [0x6000]
        mov cr3, eax
        mov byte [0x7000], 2
        mov byte [0x7000], 5
        hlt

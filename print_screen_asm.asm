[bits 64]
org 0xA000

push rax
push rdx
push rdi
push rsi
push rcx
push rbx
push r8
push r9
push r10

lea r9, [rel my_name]
push 7

mov rax, [0x8000]
mov rdi, [0x9000]
mov rsi, [0x2000]
mov rdx, 0
mov rcx, [0xB000]
mov r8, 0x00FF00
call rax

add rsp, 8

pop r10
pop r9
pop r8
pop rbx
pop rcx
pop rsi
pop rdi
pop rdx
pop rax

ret

align 8
my_name: db "Arshman", 0

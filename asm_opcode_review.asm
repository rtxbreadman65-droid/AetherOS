[bits 64]

push rax
push rdi
push rsi
push rbx
push rdx
push r8
push r9
push r10

mov rdi, 5
mov rdx, 5
add rdi, rdx
mov rcx, rdi

pop r10
pop r9
pop r8
pop rdx
pop rbx
pop rsi
pop rdi
pop rax

ret

.section .text.boot
.global mystart

mystart:
    xor rax, rax
    xor eax, eax
    jmp setcs                 // Jump if Long Mode is active

nonlong:
    jmp nonlong

setcs:
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx

    xor rsi, rsi
    xor rdi, rdi

    mov rsp, 0x3000
    mov rbp, rsp
    mov rsi, rsp
    and rsp, -16
    cld

    jmp not_main

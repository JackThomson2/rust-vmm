.section .text.boot
.global _start

_start:
    jmp setcs

setcs:
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx

    xor rsi, rsi
    xor rdi, rdi

    mov rsp, 0x55000
    mov rbp, rsp
    mov rsi, rsp
    and rsp, -16

    xor rax, rax
    xor rdx, rdx

    jmp not_main

.section .text.boot
.global mystart

mystart:
    jmp setcs

setcs:
    xor rax, rax
    mov rsp, __stack_top
    cld

    jmp not_main

.section .text.boot
.global mystart
.extern idt_descriptor

mystart:
    jmp setcs

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
    call load_idt

    mov rax, 0x8000
    mov dword ptr [rax], 0x1234

    xor rax, rax
    xor rdx, rdx

    jmp not_main

get_page_loc:
    lea rax, [idt]
    ret

debug_dot:
    push rax
    mov dx, 0x1000
    mov al, 0x2e
    out dx, al
    mov al, 0xa
    out dx, al
    pop rax
    ret

setup_idt:
    mov eax, 14
    shl eax, 4
    lea rdx, [page_fault_handler]

    mov [idt + eax], dx
    mov word ptr [idt + eax + 2], 0x8
    mov word ptr [idt + eax + 4], 0x8E

    shr rdx, 16
    mov [idt + eax + 6], dx

    shr rdx, 16
    mov [idt + eax + 8], edx

    ret

load_idt:
    call setup_idt
    lidt [idt_descriptor]
    sti
    ret


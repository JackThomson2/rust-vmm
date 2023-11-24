.section .code
.global page_fault_handler
.align 4

page_fault_handler:
    cld

    mov dx, 0x1000
    mov al, 0x2e
    out dx, al
    mov al, 0xa
    out dx, al

    mov dx, 0x1000
    mov al, 0x2e
    out dx, al
    mov al, 0xa
    out dx, al

    mov dx, 0x1000
    mov al, 0x2e
    out dx, al
    mov al, 0xa
    out dx, al

    iretq

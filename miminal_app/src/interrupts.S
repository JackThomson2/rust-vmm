.section .data
idt:
    .rept 14
    .word 0                # Lower 16 bits of ISR address (placeholder)
    .word 0                # Selector (placeholder)
    .byte 0                # Reserved (placeholder)
    .byte 0                # Type and attributes (placeholder)
    .word 0                # Higher 16 bits of ISR address (placeholder)
    .quad 0                # Full 64-bit address of ISR (placeholder)
    .endr

    # 14th entry for Page Fault
    .word 0                  # Offset bits 0-15
    .word 0x08               # Selector (Code Segment)
    .byte 0x00               # Reserved
    .byte 0x8E               # Type and attributes - P=1, DPL=00, Type=E (32-bit interrupt gate)
    .word 0
    .quad 0x0                # Offset bits 16-31

    .rept 240
    .word 0                # Lower 16 bits of ISR address (placeholder)
    .word 0                # Selector (placeholder)
    .byte 0                # Reserved (placeholder)
    .byte 0                # Type and attributes (placeholder)
    .word 0                # Higher 16 bits of ISR address (placeholder)
    .quad 0                # Full 64-bit address of ISR (placeholder)
    .endr

idt_end:

.global idt_descriptor
idt_descriptor:
    .word idt_end - idt - 1  # Limit (size of IDT - 1)
    .quad idt                # Base address of IDT


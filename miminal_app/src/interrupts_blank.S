.section .data
idt:
    .rept 255
    .word 0                # Lower 16 bits of ISR address (placeholder)
    .word 0                # Selector (placeholder)
    .byte 0                # Reserved (placeholder)
    .byte 0                # Type and attributes (placeholder)
    .word 0                # Higher 16 bits of ISR address (placeholder)
    .quad 0                # Full 64-bit address of ISR (placeholder)
    .endr

idt_descriptor:
    .word idt_end - idt - 1  # Limit (size of IDT - 1)
    .quad idt                # Base address of IDT

idt_end:

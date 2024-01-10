ENTRY(_start)

SECTIONS
{
    . = 0x0;
    .text : { *(.text*) }
    .bss  : { *(.bss*) }
    .data : { *(.data*) }
    .rodata : { *(.rodata*) }
    COMMON* : { *(COMMON*) }
    .shstrtab : {
       *(.shstrtab)
    }

    /DISCARD/ : {
        *(*)
    }
}

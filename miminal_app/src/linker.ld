ENTRY(_start)

SECTIONS
{
    . = 0x0;
    .all : {
       *(.text.boot*)
       *(.text*)
       *(.bss*)
       *(.data*)
       *(.rodata*)
       *(COMMON*)
       *(*)
    } :code
    .shstrtab : {
       *(.shstrtab)
    }
}

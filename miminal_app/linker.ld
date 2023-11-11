ENTRY(_start)
SECTIONS
{
    . = 0x0;
    .all : {
       *(.bss*)
       *(.text*)
       *(.data*)
       *(.rodata*)
       *(COMMON*)
    } :code
    .shstrtab : {
       *(.shstrtab)
    }
    /DISCARD/ : {
      *(*)
    }
}

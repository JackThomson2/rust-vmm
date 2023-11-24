ENTRY(mystart)

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
    } :code
    .shstrtab : {
       *(.shstrtab)
    }
    /DISCARD/ : {
      *(*)
    }
}

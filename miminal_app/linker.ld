ENTRY(mystart)

SECTIONS
{
    . = 0x0;
    .all : {
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

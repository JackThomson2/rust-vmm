ENTRY(mystart)

STACK_SIZE = 0x256; /* Define the stack size, adjust as needed */

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
    __stack_bottom = .;
    . = . + STACK_SIZE;
    __stack_top = .;
}


/* SECTIONS */
/* { */
/*   . = 0x0; */
/*   .text : { */
/*     entry.o(.text.boot) */
/*     *(.text.boot) */
/*     *(.text) */
/*     *(.data) */
/*     *(.rodata) */
/*     __bss_start = .; */
/*     /1* COMMON vs BSS: https://stackoverflow.com/questions/16835716/bss-vs-common-what-goes-where *1/ */
/*     *(.bss) */
/*     *(COMMON) */
/*     __bss_end = .; */
/*   } */

/*   /DISCARD/ : { */
/*     *(.eh_frame) */
/*   } */
/*   __stack_bottom = .; */
/*   . = . + STACK_SIZE; */
/*   __stack_top = .; */
/* } */

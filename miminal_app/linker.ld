OUTPUT_FORMAT(binary)

SECTIONS {
    .boot     : { *(.startboot) }
    .text     : { *(.text) }
    .data     : { *(.data) }
    .rodata   : { *(.rodata) }
    /DISCARD/ : { *(.*) }
}

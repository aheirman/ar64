MEMORY {
  rom (rx) : ORIGIN = 0x0, LENGTH = 512
  ram (rw) : ORIGIN = 512, LENGTH = 512
}

ENTRY(_start)

SECTIONS {
  .text : { 
    . = ALIGN(4);
    *(.text);
    *(.text.*)
    }
    .rodata : { *(.rodata); *(.rodata.*) }

  .bss : { *(.bss); *(.bss.*) }
  .data : { *(.data); *(.data.*) }

  .stack (NOLOAD) : {
    . = ALIGN(8);
    _sstack = .;
    KEEP(*(.stack_buffer))
    . = ALIGN(8);
    _estack = .;
  } > ram
}
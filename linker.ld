ENTRY(kernel_main)

SECTIONS
{
    /* File ko zero offset se shuru karo */
    . = 0x02000000;               /* 0x100000; */

    /* Sab se pehle hamara entry function aana chahiye, bina kisi nakhre ke! */
    .text : {
        *(.text_start)
        *(.text .text.*)
    }

    /* Baaki normal sections */
    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : { *(.bss .bss.*) }

    /* 🔥 Yeh sab se important hai: Fuzool metadata ko delete karo */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}

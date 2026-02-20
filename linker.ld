
/* Script de enlazado para Limine */
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

ENTRY(_start)

PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)); /* RX */
    rodata  PT_LOAD    FLAGS((1 << 2));            /* R */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)); /* RW */
    dynamic PT_DYNAMIC FLAGS((1 << 1) | (1 << 2)); /* RW */
}

SECTIONS
{
    /* El kernel se carga en esta dirección */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    .data : {
        *(.data .data.*)
    } :data

    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    } :data

    /* Secciones especiales para Limine */
    .requests : {
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :rodata

    /* Discard */
    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
        *(.note*)
    }
}

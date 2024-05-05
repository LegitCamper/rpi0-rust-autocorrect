ENTRY(main)
 
SECTIONS
{
    . = 0x8000;
    .text :
    {
        *(.text.main)
        *(.text*)
    }
    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096);
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096);
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    .symtab :
    {
        *(.symtab)
    }
    .strtab :
    {
        *(.strtab)
    }
    .shstrtab :
    {
        *(.shstrtab)
    }
    .comment :
    {
        *(.comment)
    }
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }
    . = ALIGN(4096); 
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}

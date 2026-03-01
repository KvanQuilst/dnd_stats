MEMORY
{
    /* These values correspond to the NXP LPC1114FN28/102 */
    FLASH : ORIGIN = 0x00000000, LENGTH = 32k
    SRAM  : ORIGIN = 0x10000000, LENGTH = 4k
}

ENTRY(VectorReset);

EXTERN(RESET_VECTOR);

SECTIONS
{
    .vector ORIGIN(FLASH) :
    {
        /* Stack Pointer */
        LONG(ORIGIN(SRAM) + LENGTH(SRAM));

        /* Reset Vector */
        KEEP(*(.vector.reset));
    } > FLASH

    .text :
    {
        *(.text .text.*);
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
    } > FLASH

    .bss :
    {
        __bss_start = .;
        *(.bss);
        __bss_end = .;
    } > SRAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        __data_start = .;
        *(.data .data.*);
        __data_end = .;
    } > SRAM

    __data_lma = LOADADDR(.data);

    /DISCARD/ :
    {
      *(.ARM.exidx . ARM.exidx.*);
    }
}

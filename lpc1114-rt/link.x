MEMORY
{
    /* These values correspond to the NXP LPC1114FN28/102 */
    FLASH : ORIGIN = 0x00000000, LENGTH = 32k
    SRAM  : ORIGIN = 0x10000000, LENGTH = 4k
}

ENTRY(__reset);

SECTIONS
{
    .vector ORIGIN(FLASH) :
    {
        /* Stack Pointer */
        LONG(ORIGIN(SRAM) + LENGTH(SRAM));
        . = ALIGN(4);

        /* Reset, first entry of Exception Vector */
        KEEP(*(.vector.reset));
        . = ALIGN(4);

        /* Exception Vector */
        KEEP(*(.vector.exceptions));
        . = ALIGN(4);
    } > FLASH

    .text :
    {
        *(.text .text.*);
        . = ALIGN(4);
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } > FLASH

    .bss :
    {
        __bss_start = .;
        *(.bss);
        . = ALIGN(4);
        __bss_end = .;
    } > SRAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        __data_start = .;
        *(.data .data.*);
        . = ALIGN(4);
        __data_end = .;
    } > SRAM

    /DISCARD/ :
    {
      *(.ARM.exidx . ARM.exidx.*);
    }
}

__data_lma = LOADADDR(.data);

/*PROVIDE(__initial_sp = ORIGIN(SRAM) + LENGTH(SRAM));*/
PROVIDE(exception_nmi = exception_default_handler);
PROVIDE(exception_hardfault = exception_default_handler);
PROVIDE(exception_svcall = exception_default_handler);
PROVIDE(exception_pendsv = exception_default_handler);
PROVIDE(exception_systick = exception_default_handler);

/*PROVIDE(__VECTOR_CHECKSUM = 0 - __initial_sp
                                - (__reset + 1)
                                - (exception_nmi + 1)
                                - (exception_hardfault + 1));
*/

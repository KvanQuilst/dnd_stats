#![no_std]
#![allow(unused)]

use core::arch::global_asm;
use core::panic::PanicInfo;

pub mod gpio;

//use pins::gpio2;

global_asm!(
    ".text
     
     .syntax unified
     .global __bss_start
     .global __bss_end

     .global __data_start
     .global __data_end
     .global __data_lma

     .global main
     .global VectorReset

     .global __reset
     .type __reset,%function
     .thumb_func
     __reset:

     __init_bss:
        movs r2, #0         // Zero r2
        ldr r0, =__bss_start
        ldr r1, =__bss_end

    1:
        cmp r0, r1          // Reach end of .bss?
        beq __init_data     // Then call __init_data
        strb r2, [r0]       // Else, 0 bss[x]
        adds r0, #1         // Increment r0 pointer
        b 1b                // Loop

    __init_data:
        ldr r0, =__data_start
        ldr r1, =__data_end
        ldr r2, =__data_lma

    1:
        cmp r0, r1      // Reached end of .data?
        beq __main__    // Then call __main__
        ldrb r3, [r2]   // Else, store flash_data[x] in r3
        strb r3, [r0]   // Put r3 in sram_data[x]
        adds r0, #1     // Increment r0 pointer
        adds r2, #1     // Increment r2 pointer
        b 1b            // Loop

    __main__:
    ldr r0, =main
    bx r0"
);

unsafe extern "C" {
    fn __reset();
    fn exception_nmi();
    fn exception_hardfault();
    fn exception_svcall();
    fn exception_pendsv();
    fn exception_systick();
}

#[unsafe(link_section = ".vector.reset")]
#[unsafe(no_mangle)]
pub static RESET: unsafe extern "C" fn() = __reset;

pub union Exception {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[unsafe(link_section = ".vector.exceptions")]
#[unsafe(no_mangle)]
pub static EXCEPTIONS: [Exception; 14] = [
    // Reset 1
    Exception {
        handler: exception_nmi, // 2
    },
    Exception {
        handler: exception_hardfault, // 3
    },
    Exception { reserved: 0 }, // 4
    Exception { reserved: 0 }, // 5
    Exception { reserved: 0 }, // 6
    Exception { reserved: 0 }, // 7 - Checksum, run lpc1114_checksum.py on bin
    Exception { reserved: 0 }, // 8
    Exception { reserved: 0 }, // 9
    Exception { reserved: 0 }, // 10
    Exception {
        handler: exception_svcall, // 11
    },
    Exception { reserved: 0 }, // 12
    Exception { reserved: 0 }, // 13
    Exception {
        handler: exception_pendsv, // 14
    },
    Exception {
        handler: exception_systick, // 15
    },
];

#[unsafe(no_mangle)]
pub extern "C" fn exception_default_handler() {
    loop {}
}

#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// Entry Point Macro
#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;
            f();
        }
    };
}

#![no_std]
#![no_main]

use core::arch::asm;
use lpc1114_rt::entry_point;
use lpc1114_rt::gpio::{
    GpioPin,
    GpioPort,
};

fn main() -> ! {
    let pin = GpioPin::new().into_port0().into_pin7();
    GpioPort::set_dir_output(&pin);
    let mut i;

    loop {
        GpioPort::set_data_high(&pin);

        i = 0u16;
        while i < 10000u16 {
            unsafe { asm!("nop"); }
            i += 1u16;
        }

        GpioPort::set_data_low(&pin);

        i = 0u16;
        while i < 10000u16 {
            unsafe { asm!("nop"); }
            i += 1u16;
        }
    }
}
entry_point!(main);

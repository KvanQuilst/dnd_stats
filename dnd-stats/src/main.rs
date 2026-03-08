#![no_std]
#![no_main]

use core::arch::asm;
use lpc1114_rt::entry_point;
use lpc1114_rt::pins::gpio::{
    GpioCfg,
    GpioPort,
};

entry_point!(main);
fn main() -> ! {
    loop {
        let gpio0_7 = GpioCfg::new(GpioPort::Port0, 7);
        let gpio0_7_out = gpio0_7.into_dir_output();
        let gpio0_7_hi = gpio0_7_out.into_data_high();

        for _n in 1..5000 {
            unsafe { asm!("nop"); }
        }
        gpio0_7_hi.into_data_low();
        for _n in 1..5000 {
            unsafe { asm!("nop"); }
        }
    }
}

#![allow(unused)]

use core::ptr::{read_volatile, write_volatile};

//
// GpioCfg States
//
macro_rules! add_state {
    ($trait:ty, $state:ident) => {
        pub struct $state;
        impl $trait for $state {}
    };
}

pub struct NA;

// Direction
pub trait Direction {}
add_state!(Direction, Input);
add_state!(Direction, Output);
impl Direction for NA {}

// Data
pub trait Data {}
add_state!(Data, PullHigh);
add_state!(Data, PullLow);

//
// GpioCfg Init
//
pub struct GpioCfg<D: Direction, Da: Data> {
    gpio: GpioPeripheral,
    direction: D,
    data: Da,
}

impl GpioCfg<Input, PullLow> {
    #[inline(always)]
    pub fn new(port: GpioPort, pin: u8) -> GpioCfg<Input, PullLow> {
        GpioCfg {
            gpio: GpioPeripheral {
                port: port,
                pin: pin,
            },
            direction: Input,
            data: PullLow,
        }
    }
}

//
// GpioCfg State Machine
//
impl GpioCfg<Input, PullLow> {
    #[inline(always)]
    pub fn into_dir_output(self) -> GpioCfg<Output, PullLow> {
        let mut port = Port::new(&self.gpio.port);
        port.set_dir_output(&self.gpio);
        GpioCfg {
            gpio: self.gpio,
            direction: Output,
            data: PullLow,
        }
    }
}

impl GpioCfg<Output, PullLow> {
    #[inline(always)]
    pub fn into_dir_input(self) -> GpioCfg<Input, PullLow> {
        let mut port = Port::new(&self.gpio.port);
        port.set_dir_input(&self.gpio);
        GpioCfg {
            gpio: self.gpio,
            direction: Input,
            data: PullLow,
        }
    }
}

impl<D: Direction> GpioCfg<D, PullLow> {
    #[inline(always)]
    pub fn into_data_high(self) -> GpioCfg<NA, PullHigh> {
        let mut port = Port::new(&self.gpio.port);
        port.set_data_high(&self.gpio);
        GpioCfg {
            gpio: self.gpio,
            direction: NA,
            data: PullHigh,
        }
    }
}

impl<D: Direction> GpioCfg<D, PullHigh> {
    #[inline(always)]
    pub fn into_data_low(self) -> GpioCfg<NA, PullLow> {
        let mut port = Port::new(&self.gpio.port);
        port.set_data_low(&self.gpio);
        GpioCfg {
            gpio: self.gpio,
            direction: NA,
            data: PullLow,
        }
    }
}

//
// Gpio Register Logic
//
struct GpioPeripheral { 
    port: GpioPort,
    pin: u8,
}

struct Port {
    reg: &'static mut GpioRegister,
}

pub enum GpioPort {
    Port0,
    Port1,
}

impl Port {
    #[inline]
    fn new(port: &GpioPort) -> Port {
        let addr = match port {
            GpioPort::Port0 => 0x5000_0000,
            GpioPort::Port1 => 0x5001_0000,
        };
        Port {
            reg: unsafe { &mut *(addr as *mut GpioRegister) }
        }
    }

    #[inline(always)]
    fn set_dir_input(&mut self, pin: &GpioPeripheral) {
        unsafe {
            let dir = read_volatile(&mut self.reg.dir);
            write_volatile(&mut self.reg.dir, dir & !(1 << pin.pin));
        }
    }

    #[inline(always)]
    fn set_dir_output(&mut self, pin: &GpioPeripheral) {
        unsafe {
            let dir = read_volatile(&mut self.reg.dir);
            write_volatile(&mut self.reg.dir, dir | (1 << pin.pin));
        }
    }

    #[inline(always)]
    fn set_data_high(&mut self, pin: &GpioPeripheral) {
        unsafe { write_volatile(&mut self.reg.data[1 << pin.pin], 0xFFFF); }
    }

    #[inline(always)]
    fn set_data_low(&mut self, pin: &GpioPeripheral) {
        unsafe { write_volatile(&mut self.reg.data[1 << pin.pin], 0x0000); }
    }
}

#[repr(C)]
#[repr(align(4))]
struct GpioRegister {
    data: [u32; 1 << 12],
    rsv0: [u32; 1 << 12],
    dir: u32,
    is: u32,
    be: u32,
    ev: u32,
    e: u32,
    ris: u32,
    mis: u32,
    ic: u32,
}

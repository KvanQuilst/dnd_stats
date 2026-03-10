#![allow(unused)]

use core::ptr::{read_volatile, write_volatile};

macro_rules! add_state {
    ($trait:ty, $state:ident) => {
        pub struct $state;
        impl $trait for $state {}
    };
}

pub trait Pin {}
add_state!(Pin, NoPin);
add_state!(Pin, Pin0);
add_state!(Pin, Pin1);
add_state!(Pin, Pin2);
add_state!(Pin, Pin3);
add_state!(Pin, Pin4);
add_state!(Pin, Pin5);
add_state!(Pin, Pin6);
add_state!(Pin, Pin7);
add_state!(Pin, Pin8);
add_state!(Pin, Pin9);
add_state!(Pin, Pin10);
add_state!(Pin, Pin11);

pub trait Port {}
add_state!(Port, NoPort);
add_state!(Port, Port0);
add_state!(Port, Port1);

pub struct GpioPin<Po: Port, Pi: Pin> {
    _port: Po,
    _pin: Pi,
    pin_num: u8,
}

impl GpioPin<NoPort, NoPin> {
    #[inline(always)]
    pub fn new() -> GpioPin<NoPort, NoPin> {
        GpioPin { _port: NoPort, _pin: NoPin, pin_num: 0xFF }
    }
}

macro_rules! into_port {
    ($func:ident, $port:ident) => {
        impl GpioPin<NoPort, NoPin> {
            #[inline(always)]
            pub fn $func(self) -> GpioPin<$port, NoPin> {
                GpioPin { _port: $port, _pin: NoPin, pin_num: 0xFF }
            }
        }
    };
}
into_port!(into_port0, Port0);
into_port!(into_port1, Port1);

macro_rules! into_pin {
    ($func:ident, $port:ident, $pin:ident, $pin_num:literal) => {
        impl GpioPin<$port, NoPin> {
            #[inline(always)]
            pub fn $func(self) -> GpioPin<$port, $pin> {
                GpioPin { _port: $port, _pin: $pin, pin_num: $pin_num }
            }
        }
    };
}
into_pin!(into_pin0, Port0, Pin0, 0);
into_pin!(into_pin1, Port0, Pin1, 1);
into_pin!(into_pin2, Port0, Pin2, 2);
into_pin!(into_pin3, Port0, Pin3, 3);
into_pin!(into_pin4, Port0, Pin4, 4);
into_pin!(into_pin5, Port0, Pin5, 5);
into_pin!(into_pin6, Port0, Pin6, 6);
into_pin!(into_pin7, Port0, Pin7, 7);
into_pin!(into_pin8, Port0, Pin8, 8);
into_pin!(into_pin9, Port0, Pin9, 9);
into_pin!(into_pin10, Port0, Pin10, 10);
into_pin!(into_pin11, Port0, Pin11, 11);

into_pin!(into_pin0, Port1, Pin0, 0);
into_pin!(into_pin1, Port1, Pin1, 1);
into_pin!(into_pin2, Port1, Pin2, 2);
into_pin!(into_pin3, Port1, Pin3, 3);
into_pin!(into_pin4, Port1, Pin4, 4);
into_pin!(into_pin5, Port1, Pin5, 5);
into_pin!(into_pin6, Port1, Pin6, 6);
into_pin!(into_pin7, Port1, Pin7, 7);
into_pin!(into_pin8, Port1, Pin8, 8);
into_pin!(into_pin9, Port1, Pin9, 9);

pub struct GpioPort<P: Port> {
    addr: u32,
    _port: P,
}
//reg: &'static mut GpioRegister 

impl<P: Port> GpioPort<P> {
    #[inline(always)]
    fn grab_port0() -> GpioPort<Port0> {
       GpioPort {
           addr: 0x5000_0000,
           _port: Port0,
       }
    }

    #[inline(always)]
    fn grab_port1() -> GpioPort<Port1> {
        GpioPort {
            addr: 0x5001_0000,
            _port: Port1,
        }
    }
}

macro_rules! port0_reg {
    () => {
        unsafe { &mut *(Self::grab_port0().addr as *mut GpioRegister) }
    };
}
macro_rules! port1_reg {
    () => {
        unsafe { &mut *(Self::grab_port1().addr as *mut GpioRegister) }
    };
}

impl GpioPort<Port0> {
    #[inline(always)]
    pub fn set_dir_input<P: Pin>(pin: &GpioPin<Port0, P>) {
        unsafe {
            let reg = port0_reg!();
            let dir = read_volatile(&mut reg.dir);
            write_volatile(&mut reg.dir, dir & !(1 << pin.pin_num));
        }
    }

    #[inline(always)]
    pub fn set_dir_output<P: Pin>(pin: &GpioPin<Port0, P>) {
        unsafe {
            let reg = port0_reg!();
            let dir = read_volatile(&mut reg.dir);
            write_volatile(&mut reg.dir, dir | (1 << pin.pin_num));
        }
    }

    #[inline(always)]
    pub fn set_data_high<P: Pin>(pin: &GpioPin<Port0, P>) {
        unsafe {
            let reg = port0_reg!();
            write_volatile(&mut reg.data[1 << pin.pin_num], 0xFFFF);
        }
    }

    #[inline(always)]
    pub fn set_data_low<P: Pin>(pin: &GpioPin<Port0, P>) {
        unsafe {
            let reg = port0_reg!();
            write_volatile(&mut reg.data[1 << pin.pin_num], 0x0000);
        }
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

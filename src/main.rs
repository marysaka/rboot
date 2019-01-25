#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(underscore_const_names)]

#[macro_use]
extern crate static_assertions;

extern crate register;

#[macro_use]
extern crate enum_primitive;

pub mod rt;
pub mod tegra210;

use tegra210::gpio::{Gpio, GpioConfig, GpioPort};
use tegra210::*;

const APB: *const apb::AMBAPeripheralBus = 0x70000000 as *const apb::AMBAPeripheralBus;

entry!(main);

const P2371_2180_GPIO_CONFIG: [(Gpio, GpioConfig); 59] = [
    (Gpio(GpioPort::A, 5), GpioConfig::Input),
    (Gpio(GpioPort::B, 0), GpioConfig::Input),
    (Gpio(GpioPort::B, 1), GpioConfig::Input),
    (Gpio(GpioPort::B, 2), GpioConfig::Input),
    (Gpio(GpioPort::B, 3), GpioConfig::Input),
    (Gpio(GpioPort::C, 0), GpioConfig::Input),
    (Gpio(GpioPort::C, 1), GpioConfig::Input),
    (Gpio(GpioPort::C, 2), GpioConfig::Input),
    (Gpio(GpioPort::C, 3), GpioConfig::Input),
    (Gpio(GpioPort::C, 4), GpioConfig::Input),
    (Gpio(GpioPort::E, 4), GpioConfig::Input),
    (Gpio(GpioPort::E, 5), GpioConfig::Input),
    (Gpio(GpioPort::E, 6), GpioConfig::Input),
    (Gpio(GpioPort::H, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 2), GpioConfig::Input),
    (Gpio(GpioPort::H, 3), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 4), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 5), GpioConfig::Input),
    (Gpio(GpioPort::H, 6), GpioConfig::Input),
    (Gpio(GpioPort::H, 7), GpioConfig::Input),
    (Gpio(GpioPort::I, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::I, 1), GpioConfig::Input),
    (Gpio(GpioPort::I, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::K, 4), GpioConfig::Input),
    (Gpio(GpioPort::K, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::K, 6), GpioConfig::Input),
    (Gpio(GpioPort::K, 7), GpioConfig::Input),
    (Gpio(GpioPort::L, 1), GpioConfig::Input),
    (Gpio(GpioPort::S, 4), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 6), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 7), GpioConfig::OutputLow),
    (Gpio(GpioPort::T, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::T, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::U, 2), GpioConfig::Input),
    (Gpio(GpioPort::U, 3), GpioConfig::Input),
    (Gpio(GpioPort::V, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 3), GpioConfig::Input),
    (Gpio(GpioPort::V, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 6), GpioConfig::OutputLow),
    (Gpio(GpioPort::X, 0), GpioConfig::Input),
    (Gpio(GpioPort::X, 1), GpioConfig::Input),
    (Gpio(GpioPort::X, 2), GpioConfig::Input),
    (Gpio(GpioPort::X, 3), GpioConfig::Input),
    (Gpio(GpioPort::X, 4), GpioConfig::Input),
    (Gpio(GpioPort::X, 5), GpioConfig::Input),
    (Gpio(GpioPort::X, 6), GpioConfig::Input),
    (Gpio(GpioPort::X, 7), GpioConfig::Input),
    (Gpio(GpioPort::Y, 0), GpioConfig::Input),
    (Gpio(GpioPort::Y, 1), GpioConfig::Input),
    (Gpio(GpioPort::Z, 0), GpioConfig::Input),
    (Gpio(GpioPort::Z, 2), GpioConfig::Input),
    (Gpio(GpioPort::Z, 3), GpioConfig::OutputLow),
    (Gpio(GpioPort::BB, 0), GpioConfig::Input),
    (Gpio(GpioPort::BB, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::BB, 3), GpioConfig::Input),
    (Gpio(GpioPort::CC, 1), GpioConfig::Input),
];

fn pinmux_init() {
    // clear clamping
    unsafe { (*APB).misc.pp.PINMUX_GLOBAL.set(0) };

    for entry in P2371_2180_GPIO_CONFIG.iter() {
        entry.0.config(entry.1);
    }
}

fn main() -> () {
    pinmux_init();
}

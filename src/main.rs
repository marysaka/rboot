#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(underscore_const_names)]

extern crate embedded_serial;

#[macro_use]
extern crate enum_primitive;

extern crate register;

#[macro_use]
extern crate static_assertions;


pub mod rt;
pub mod tegra210;
pub mod serial;

use tegra210::board;
use tegra210::*;

use embedded_serial::ImmutBlockingTx;

const APB: *const apb::AMBAPeripheralBus = 0x7000_0000 as *const apb::AMBAPeripheralBus;

entry!(main);

fn pinmux_init() {
    // clear clamping
    unsafe { (*APB).misc.pp.PINMUX_GLOBAL.set(0) };

    // configure GPIO
    for entry in board::p2371_2180::GPIO_CONFIG.iter() {
        entry.0.config(entry.1);
    }

    // configure PINGRP
    for entry in board::p2371_2180::PINGRP_CONFIG.iter() {
        entry.0.config(
            entry.1, entry.2, entry.3, entry.4, entry.5, entry.6, entry.7,
        );
    }

    // TODO: configure DRVCFG
}

fn main() {
    pinmux_init();

    let uart_a = uart::UART::A;

    uart_a.init(115200);
    uart_a.puts("Hello World from Rust\r\n");
}

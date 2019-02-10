#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(underscore_const_names)]

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate log;

extern crate register;

#[macro_use]
extern crate static_assertions;

extern crate num_traits;

pub mod exception_vectors;
pub mod logger;
pub mod mmu;
pub mod rt;
pub mod tegra210;

use core::fmt::Write;

use crate::tegra210::board;
use crate::tegra210::*;

use log::Level;

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

extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
    static _stack_top: u8;
}

fn log_init() {
    let mut uart_a = &mut uart::UART::A;
    uart_a.init(115_200);

    let res = logger::init(logger::Type::A, Level::Trace);

    writeln!(&mut uart_a, "{:?}\r", res).unwrap();
}

fn main() {
    pinmux_init();
    log_init();

    let current_el: u32;
    unsafe { asm!("mrs $0, CurrentEL" : "=r"(current_el) ::: "volatile") }
    info!("Executing in EL: {}", current_el >> 2);

    let core_id: u64;
    unsafe { asm!("mrs $0, mpidr_el1" : "=r"(core_id) ::: "volatile") }

    info!("Core id: {}", core_id & 0x3);
}

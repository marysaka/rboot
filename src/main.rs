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
pub mod utils;

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
    let uart_a = &uart::UART::A;
    uart_a.init(115_200);

    logger::init(logger::Type::A, Level::Trace).unwrap();
}

fn main() {
    pinmux_init();
    log_init();

    info!("Hello World");

    let current_el = utils::get_current_el();
    trace!("Executing in EL: {}", current_el);

    let core_id: u64;
    unsafe { asm!("mrs $0, mpidr_el1" : "=r"(core_id) ::: "volatile") }

    trace!("Core id: {}", core_id & 0x3);

    let ptr = 0x80800000 as *mut u64;
    let ptr_value = ptr as u64;

    mmu::map_normal_page(ptr_value, ptr_value, 4096, mmu::MemoryPermission::RW);
    unsafe {
        *ptr = 0xCAFEBABE;
    }
    trace!("Change permissions of page to R--");
    mmu::map_normal_page(ptr_value, ptr_value, 4096, mmu::MemoryPermission::R);

    unsafe {
        trace!("val: 0x{:X}", *ptr);
        trace!("Try another write (will data abort)");
        *ptr = 0xDEADBEEF;
        trace!("new val: 0x{:X}", *ptr);
    }
}

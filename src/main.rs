#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate libtegra;

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

use libtegra::car::Clock;
use libtegra::mc::REGISTERS as MC;
use libtegra::pmc::{powergate_partition, Partition};
use libtegra::tsec::{FalconError, Tsec};
use libtegra::uart::{Uart, BAUD_115200};
use libtegra::apb::misc::REGISTERS as APB;
use log::Level;

const TSEC: Tsec = Tsec::A;

entry!(main);

unsafe fn pinmux_init() {
    // clear clamping
    (*APB).pp.APB_MISC_PP_PINMUX_GLOBAL_0_0.set(0);

    // configure GPIO
    for entry in board::p2371_2180::GPIO_CONFIG.iter() {
        entry.0.config(entry.1);
    }

    // configure PINGRP
    for entry in board::p2371_2180::PINGRP_CONFIG.iter() {
        entry.0.set_function(entry.1);
        entry.0.set_pull(entry.2);
        entry.0.set_tristate(entry.3);
        entry.0.set_io(entry.4);
        entry.0.set_lock(entry.5);
        entry.0.set_od(entry.6);
        entry.0.set_io_hv(entry.7);
    }

    // TODO: configure DRVCFG
}

extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
    static _stack_top: u8;
}

fn log_init() {
    Uart::A.init(BAUD_115200);

    logger::init(logger::Type::A, Level::Trace).unwrap();
}

use register::mmio::*;

include!(concat!(env!("OUT_DIR"), "/falcon_fw.rs"));

fn bring_up_sors() {
    powergate_partition(Partition::SOR, false).expect("Cannot power ungate SOR");

    Clock::SOR_SAFE.enable();
    Clock::SOR0.enable();
    Clock::SOR1.enable();
    Clock::DPAUX.enable();
    Clock::DPAUX1.enable();
    Clock::MIPI_CAL.enable();
    Clock::CSI.enable();
    Clock::DSI.enable();
    Clock::DSIB.enable();

    powergate_partition(Partition::SOR, true).expect("Cannot power gate SOR");
}

fn execute_tsec_fw(
    firmware: &[u8],
    boot_vector: u32,
    mailbox0: &mut u32,
    mailbox1: &mut u32,
) -> Result<(), FalconError> {
    TSEC.load_firmware(firmware)?;
    unsafe { TSEC.boot(boot_vector, mailbox0, mailbox1) }
}

fn main() {
    unsafe { pinmux_init() };

    log_init();

    info!("Hello World");

    TSEC.init();

    bring_up_sors();

    let mut argument0 = 0;
    let mut argument1 = 0;
    let res = execute_tsec_fw(&*FALCON_FW, 0, &mut argument0, &mut argument1);

    info!("{:?}", res);
    info!("argument0: 0x{:x}", argument0);
    info!("argument1: 0x{:x}", argument1);

    TSEC.finalize();
}

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]

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
use crate::tegra210::clock::Clock;
use crate::tegra210::*;

use log::Level;

const APB: *const apb::AMBAPeripheralBus = 0x7000_0000 as *const apb::AMBAPeripheralBus;
const TSEC: *const tsec::TSEC = 0x5450_0000 as *const tsec::TSEC;

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

use register::mmio::*;

include!(concat!(env!("OUT_DIR"), "/falcon_fw.rs"));

// TODO: this should be in a bitfield.
const TSEC_IRQMSET_WDTMR: u32 = (1 << 1);
const TSEC_IRQMSET_HALT: u32 = (1 << 4);
const TSEC_IRQMSET_EXTERR: u32 = (1 << 5);
const TSEC_IRQMSET_SWGEN0: u32 = (1 << 6);
const TSEC_IRQMSET_SWGEN1: u32 = (1 << 7);

const TSEC_IRQMSET_EXT_ALL: u32 = 0xFF00;

const TSEC_IRQDEST_HALT: u32 = (1 << 4);
const TSEC_IRQDEST_EXTERR: u32 = (1 << 5);
const TSEC_IRQDEST_SWGEN0: u32 = (1 << 6);
const TSEC_IRQDEST_SWGEN1: u32 = (1 << 7);

const TSEC_IRQDEST_EXT_ALL: u32 = 0xFF00;

const TSEC_ITFEN_CTXEN: u32 = (1 << 0);
const TSEC_ITFEN_MTHDEN: u32 = (1 << 1);

fn tsec_dma_wait_idle() -> Result<(), ()> {
    let timeout = timer::get_ms() + 1000;
    let tsec_instance: &tsec::TSEC = unsafe { &(*TSEC) };


    info!("{:p}", &tsec_instance.FALCON_DMATRFCMD);
    info!("{:b}", tsec_instance.FALCON_DMATRFCMD.get());

    while (tsec_instance.FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
        if timer::get_ms() > timeout {
            info!("{:b}", tsec_instance.FALCON_DMATRFCMD.get());
            return Err(())
        }
    }

    Ok(())
}

#[derive(Debug)]
enum FalconError {
    DmaTimeout
}

fn execute_tsec_fw(fw: &[u8]) -> Result<(), FalconError> {
    let mut res = Ok(());

    Clock::HOST1X.enable();
    Clock::TSEC.enable();
    Clock::SOR_SAFE.enable();
    Clock::SOR0.enable();
    Clock::SOR1.enable();
    Clock::KFUSE.enable();

    // All clocks up, time to do something.
    let tsec_instance: &tsec::TSEC = unsafe { &(*TSEC) };

    tsec_instance.FALCON_DMACTL.set(0);
    tsec_instance.FALCON_IRQMSET.set(
        TSEC_IRQMSET_EXT_ALL
            | TSEC_IRQMSET_WDTMR
            | TSEC_IRQMSET_HALT
            | TSEC_IRQMSET_EXTERR
            | TSEC_IRQMSET_SWGEN0
            | TSEC_IRQMSET_SWGEN1,
    );

    tsec_instance.FALCON_IRQDEST.set(
        TSEC_IRQDEST_EXT_ALL
            | TSEC_IRQDEST_HALT
            | TSEC_IRQDEST_EXTERR
            | TSEC_IRQDEST_SWGEN0
            | TSEC_IRQDEST_SWGEN1,
    );

    tsec_instance.FALCON_ITFEN.set(TSEC_ITFEN_CTXEN | TSEC_ITFEN_MTHDEN);

    let dma_idle_res = tsec_dma_wait_idle();
    if dma_idle_res.is_ok() {
        info!("DMA IDLE");
    } else {
        res = Err(FalconError::DmaTimeout);
    }

    Clock::KFUSE.disable();
    Clock::SOR1.disable();
    Clock::SOR0.disable();
    Clock::SOR_SAFE.disable();
    Clock::TSEC.disable();
    Clock::HOST1X.disable();

    res
}

fn main() {
    //pinmux_init();
    log_init();

    info!("Hello World");

    let res = execute_tsec_fw(FALCON_FW);

    info!("{:?}", res);
}

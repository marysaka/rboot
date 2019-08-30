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
const MC: *const mc::MemoryController = 0x7001_9000 as *const mc::MemoryController;

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

unsafe fn tsec_dma_wait_idle() -> Result<(), ()> {
    let timeout = timer::get_ms() + 1000;
    let tsec_instance: &tsec::TSEC = unsafe { &(*TSEC) };


    info!("{:p}", &(*TSEC).FALCON_IRQSTAT);
    info!("FALCON_IRQSTAT: {:x}", (*TSEC).FALCON_IRQSTAT.get());
    info!("FALCON_DBG_STATE: {:x}", (*TSEC).FALCON_DBG_STATE.get());

    while ((*TSEC).FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
        if timer::get_ms() > timeout {
            return Err(())
        }
    }

    Ok(())
}

#[derive(Debug)]
enum FalconError {
    DmaTimeout
}

unsafe fn execute_tsec_fw(fw: &[u8]) -> Result<(), FalconError> {
    let mut res = Ok(());

    Clock::SE.enable();
    Clock::HOST1X.enable();
    Clock::TSEC.enable();
    Clock::TSECB.enable();
    Clock::SOR_SAFE.enable();
    Clock::SOR0.enable();
    Clock::SOR1.enable();
    Clock::KFUSE.enable();

    // All clocks up, time to do something.
    //let tsec_instance: &tsec::TSEC = unsafe { &(*TSEC) };

    info!("FALCON_DMACTL: {:x}", (*TSEC).FALCON_DMACTL.get());
    (*TSEC).FALCON_DMACTL.set(0);
    info!("FALCON_DMACTL: {:x}", (*TSEC).FALCON_DMACTL.get());
    (*TSEC).FALCON_IRQMSET.set(
        TSEC_IRQMSET_EXT_ALL
            | TSEC_IRQMSET_WDTMR
            | TSEC_IRQMSET_HALT
            | TSEC_IRQMSET_EXTERR
            | TSEC_IRQMSET_SWGEN0
            | TSEC_IRQMSET_SWGEN1,
    );

    (*TSEC).FALCON_IRQDEST.set(
        TSEC_IRQDEST_EXT_ALL
            | TSEC_IRQDEST_HALT
            | TSEC_IRQDEST_EXTERR
            | TSEC_IRQDEST_SWGEN0
            | TSEC_IRQDEST_SWGEN1,
    );

    (*TSEC).FALCON_ITFEN.set(TSEC_ITFEN_CTXEN | TSEC_ITFEN_MTHDEN);

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
    Clock::TSECB.disable();
    Clock::TSEC.disable();
    Clock::HOST1X.disable();

    res
}

unsafe fn pmc_powergate_partition(partition_id: u32, enabled: bool) -> Result<(), ()> {
    let pmc_powergate_toggle: &ReadWrite<u32> = &*(0x7000E430 as *mut ReadWrite<u32>);
    let pmc_powergate_status: &ReadOnly<u32> = &*(0x7000E438 as *mut ReadOnly<u32>);
    let partition_mask = 1 << partition_id;
    let state_changed = (enabled as u32) << partition_id;

    info!("{:032b}", pmc_powergate_status.get());

    if (pmc_powergate_status.get() & partition_mask) == state_changed{
        info!("parition_id: {} already {}", partition_id, enabled);
        return Ok(());
    }

    let mut i = 5001;
    while (pmc_powergate_toggle.get() & 0x100) != 0
    {
        timer::usleep(1);
        i -= 1;
        if (i < 1) {
            return Err(());
        }
    }

    pmc_powergate_toggle.set(partition_id | 0x100);

    i = 5001;
    while i > 0
    {
        if (pmc_powergate_status.get() & partition_mask) == state_changed {
            let pmc_remove_clamping_cmd: &ReadWrite<u32> = &*(0x7000E434 as *mut ReadWrite<u32>);

            pmc_remove_clamping_cmd.set(partition_mask);
            while (pmc_remove_clamping_cmd.get() & partition_mask) != 0 {}
            return Ok(());
        }

        timer::usleep(1);
        i -= 1;
    }

    Err(())
}

fn bring_up_sors() {
    unsafe { pmc_powergate_partition(17, false).expect("Cannot power ungate SOR"); }

    Clock::SOR_SAFE.enable();
    Clock::SOR0.enable();
    Clock::SOR1.enable();
    Clock::DPAUX.enable();
    Clock::DPAUX1.enable();
    Clock::MIPI_CAL.enable();
    Clock::CSI.enable();
    Clock::DSI.enable();
    Clock::DSIB.enable();

    unsafe {
        pmc_powergate_partition(17, true).expect("Cannot power ungate SOR");
    }
}

fn main() {
    //pinmux_init();
    log_init();

    info!("Hello World");

    info!("SE clock enabled: {}", Clock::SE.is_enabled());
    info!("HOST1X clock enabled: {}", Clock::HOST1X.is_enabled());
    info!("TSEC clock enabled: {}", Clock::TSEC.is_enabled());
    info!("SOR_SAFE clock enabled: {}", Clock::SOR_SAFE.is_enabled());
    info!("SOR0 clock enabled: {}", Clock::SOR0.is_enabled());
    info!("SOR1 clock enabled: {}", Clock::SOR1.is_enabled());
    info!("SOR1 clock enabled: {}", Clock::SOR1.is_enabled());
    info!("DPAUX clock enabled: {}", Clock::DPAUX.is_enabled());
    info!("DPAUX1 clock enabled: {}", Clock::DPAUX1.is_enabled());
    info!("MIPI_CAL clock enabled: {}", Clock::MIPI_CAL.is_enabled());
    info!("CSI clock enabled: {}", Clock::CSI.is_enabled());
    info!("DSI clock enabled: {}", Clock::DSI.is_enabled());
    info!("DSIB clock enabled: {}", Clock::DSIB.is_enabled());
    info!("KFUSE clock enabled: {}", Clock::KFUSE.is_enabled());

    // HOST1X is expected to be up
    Clock::HOST1X.enable();

    // Bring a good amount of power domain down (maybe TSEC is in a fucked state because of the previous bootloader?)
    /*unsafe { pmc_powergate_partition(23, false).expect("Cannot power ungate VIC"); }
    unsafe { pmc_powergate_partition(29, false).expect("Cannot power ungate VE2"); }
    unsafe { pmc_powergate_partition(2, false).expect("Cannot power ungate VE"); }
    unsafe { pmc_powergate_partition(18, false).expect("Cannot power ungate DIS"); }
    unsafe { pmc_powergate_partition(19, false).expect("Cannot power ungate DISB"); }
    unsafe { pmc_powergate_partition(27, false).expect("Cannot power ungate AUD"); }
    unsafe { pmc_powergate_partition(26, false).expect("Cannot power ungate NVJPG"); }
    unsafe { pmc_powergate_partition(25, false).expect("Cannot power ungate NVDEC"); }

    bring_up_sors();*/


    let res = unsafe { execute_tsec_fw(FALCON_FW) };

    info!("{:?}", res);
}

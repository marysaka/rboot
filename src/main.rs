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
const TSEC_IRQMSET_WDTMR: u32 = 1 << 1;
const TSEC_IRQMSET_HALT: u32 = 1 << 4;
const TSEC_IRQMSET_EXTERR: u32 = 1 << 5;
const TSEC_IRQMSET_SWGEN0: u32 = 1 << 6;
const TSEC_IRQMSET_SWGEN1: u32 = 1 << 7;

const TSEC_IRQMSET_EXT_ALL: u32 = 0xFF00;

const TSEC_IRQDEST_HALT: u32 = 1 << 4;
const TSEC_IRQDEST_EXTERR: u32 = 1 << 5;
const TSEC_IRQDEST_SWGEN0: u32 = 1 << 6;
const TSEC_IRQDEST_SWGEN1: u32 = 1 << 7;

const TSEC_IRQDEST_EXT_ALL: u32 = 0xFF00;

const TSEC_ITFEN_CTXEN: u32 = 1 << 0;
const TSEC_ITFEN_MTHDEN: u32 = 1 << 1;

const TSEC_FALCON_DMATRFCMD_IMEM: u32 = 1 << 4;
const TSEC_FALCON_DMATRFCMD_SIZE_256B: u32 = 6 << 8;

const TSEC_FALCON_CPUCTL_IINVAL: u32 = 1 << 0;
const TSEC_FALCON_CPUCTL_STARTCPU: u32 = 1 << 1;
const TSEC_FALCON_CPUCTL_SRESET: u32 = 1 << 2;
const TSEC_FALCON_CPUCTL_HRESET: u32 = 1 << 3;
const TSEC_FALCON_CPUCTL_HALTED: u32 = 1 << 4;
const TSEC_FALCON_CPUCTL_STOPPED: u32 = 1 << 5;
const TSEC_FALCON_CPUCTL_ALIAS_EN: u32 = 1 << 6;

const TSEC_FW_ALIGN_BITS: usize = 8;
const TSEC_FW_ALIGN: usize = 1 << TSEC_FW_ALIGN_BITS;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FalconExceptionCause {
    Trap0,
    Trap1,
    Trap2,
    Trap3,
    InvalidOpcode,
    AuthenticationEntry,
    PageMiss,
    PageMultipleMiss,
    BreakpointHit,
}

impl From<u8> for FalconExceptionCause {
    fn from(exception_cause: u8) -> FalconExceptionCause {
        match exception_cause {
            0 => FalconExceptionCause::Trap0,
            1 => FalconExceptionCause::Trap1,
            2 => FalconExceptionCause::Trap2,
            3 => FalconExceptionCause::Trap3,
            8 => FalconExceptionCause::InvalidOpcode,
            9 => FalconExceptionCause::AuthenticationEntry,
            10 => FalconExceptionCause::PageMiss,
            11 => FalconExceptionCause::PageMultipleMiss,
            15 => FalconExceptionCause::BreakpointHit,
            n => unreachable!("Got unexpected FalconExceptionCause: {}", n),
        }
    }
}

impl From<FalconExceptionCause> for u8 {
    fn from(exception_cause: FalconExceptionCause) -> u8 {
        exception_cause as u8
    }
}

assert_eq_size!(FalconExceptionCause, u8);

#[derive(Debug)]
pub enum FalconError {
    DmaTimeout,
    FirmwareMisaligned,
    Exception(u32, FalconExceptionCause),
}

fn tsec_dma_wait_idle(tsec: &tsec::TSEC) -> Result<(), FalconError> {
    let timeout = timer::get_ms() + 1000;

    while (tsec.FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
        if timer::get_ms() > timeout {
            return Err(FalconError::DmaTimeout);
        }
    }

    Ok(())
}

fn tsec_dma_copy_to_internal(tsec: &tsec::TSEC, firmware: &[u8]) -> Result<(), FalconError> {
    let firmware_addr = firmware.as_ptr() as usize;
    if (firmware_addr % TSEC_FW_ALIGN) != 0 {
        return Err(FalconError::FirmwareMisaligned);
    }

    tsec.FALCON_DMATRFBASE
        .set((firmware_addr >> TSEC_FW_ALIGN_BITS) as u32);

    for (index, _) in firmware.chunks(TSEC_FW_ALIGN).enumerate() {
        let base = (index * TSEC_FW_ALIGN) as u32;
        let offset = base;

        tsec.FALCON_DMATRFMOFFS.set(offset);
        tsec.FALCON_DMATRFFBOFFS.set(base);
        tsec.FALCON_DMATRFCMD.set(TSEC_FALCON_DMATRFCMD_IMEM);

        tsec_dma_wait_idle(tsec)?;
    }

    Ok(())
}

fn execute_tsec_fw(
    firmware: &[u8],
    boot_vector_address: u32,
    argument0: &mut u32,
    argument1: &mut u32,
) -> Result<(), FalconError> {
    let mut res;

    Clock::SE.enable();
    Clock::HOST1X.enable();
    Clock::TSEC.enable();
    Clock::TSECB.enable();
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

    // Enable all interfaces
    tsec_instance
        .FALCON_ITFEN
        .set(TSEC_ITFEN_CTXEN | TSEC_ITFEN_MTHDEN);

    res = tsec_dma_wait_idle(tsec_instance);
    if res.is_ok() {
        // Load the firmware
        res = tsec_dma_copy_to_internal(tsec_instance, firmware);

        if res.is_ok() {
            // Setup the mailboxes
            tsec_instance.FALCON_MAILBOX0.set(*argument0);
            tsec_instance.FALCON_MAILBOX0.set(*argument1);

            // Setup the boot vector
            tsec_instance.FALCON_BOOTVEC.set(boot_vector_address);

            // Start the CPU!
            tsec_instance.FALCON_CPUCTL.set(TSEC_FALCON_CPUCTL_STARTCPU);

            res = tsec_dma_wait_idle(tsec_instance);

            if res.is_ok() {
                // Wait for the CPU to be halted
                while tsec_instance.FALCON_CPUCTL.get() != TSEC_FALCON_CPUCTL_HALTED {}

                // Check that the CPU hasn't crashed
                let exception_info = tsec_instance.FALCON_EXCI.get();

                if exception_info != 0 {
                    let pc = exception_info & 0x80000;
                    let exception = FalconExceptionCause::from((exception_info >> 20) as u8 & 0xF);

                    res = Err(FalconError::Exception(pc, exception));
                }

                *argument0 = tsec_instance.FALCON_MAILBOX0.get();
                *argument1 = tsec_instance.FALCON_MAILBOX1.get();
            }
        }
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

    if (pmc_powergate_status.get() & partition_mask) == state_changed {
        info!("parition_id: {} already {}", partition_id, enabled);
        return Ok(());
    }

    let mut i = 5001;
    while (pmc_powergate_toggle.get() & 0x100) != 0 {
        timer::usleep(1);
        i -= 1;
        if i < 1 {
            return Err(());
        }
    }

    pmc_powergate_toggle.set(partition_id | 0x100);

    i = 5001;
    while i > 0 {
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
    unsafe {
        pmc_powergate_partition(17, false).expect("Cannot power ungate SOR");
    }

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

    // HOST1X is expected to be up
    Clock::HOST1X.enable();

    bring_up_sors();

    let mut argument0 = 0;
    let mut argument1 = 0;
    let res = execute_tsec_fw(&FALCON_FW.value, 0, &mut argument0, &mut argument1);

    info!("{:?}", res);
    info!("argument0: 0x{:x}", argument0);
    info!("argument1: 0x{:x}", argument1);
}

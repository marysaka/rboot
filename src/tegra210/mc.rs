use register::mmio::*;
use static_assertions::assert_eq_size;

#[allow(non_snake_case)]
#[repr(C)]
// TODO: define all registers
// TODO: checks if they are all rw
pub struct MemoryController {
    pub INTSTATUS: ReadWrite<u32>,          // 0x0
    pub INTMASK: ReadWrite<u32>,            // 0x4
    pub ERR_STATUS: ReadWrite<u32>,         // 0x8
    pub ERR_ADDR: ReadWrite<u32>,           // 0xc
    pub SMMU_CONFIG: ReadWrite<u32>,        // 0x10
    pub SMMU_TLB_CONFIG: ReadWrite<u32>,    // 0x14
    pub SMMU_PTC_CONFIG: ReadWrite<u32>,    // 0x18
    pub SMMU_PTB_ASID: ReadWrite<u32>,      // 0x1c
    pub SMMU_PTB_DATA: ReadWrite<u32>,      // 0x20
    UNKNOWN_0x24: [u8; 0xC],                // 0x24
    pub SMMU_TLB_FLUSH: ReadWrite<u32>,     // 0x30
    pub SMMU_PTC_FLUSH: ReadWrite<u32>,     // 0x34
    pub SMMU_ASID_SECURITY: ReadWrite<u32>, // 0x38
    // TODO: do not ignore this
    IGNORED_0: [u8; 0x30],                         // 0x40
    pub SECURITY_CFG0: ReadWrite<u32>,             // 0x70
    pub SECURITY_CFG1: ReadWrite<u32>,             // 0x74
    IGNORED_1: [u8; 0x1B4],                        // 0x78
    pub SMMU_TRANSLATION_ENABLE_0: ReadWrite<u32>, // 0x228
    pub SMMU_TRANSLATION_ENABLE_1: ReadWrite<u32>, // 0x22c
    pub SMMU_TRANSLATION_ENABLE_2: ReadWrite<u32>, // 0x230
    pub SMMU_TRANSLATION_ENABLE_3: ReadWrite<u32>, // 0x234
    pub SMMU_AFI_ASID: ReadWrite<u32>,             // 0x238
    pub SMMU_AVPC_ASID: ReadWrite<u32>,            // 0x23c
}

pub mod smmu {
    pub const DISABLE_SMMU: u32 = 0;
    pub const ENABLE_SMMU: u32 = 1;
    pub const TRANSLATION_DISABLE: u32 = 0;
    pub const TRANSLATION_ENABLE: u32 = !TRANSLATION_DISABLE;
    pub const TLB_CONFIG_RESET_VALUE: u32 = 0x2000_0010;
    pub const PTC_CONFIG_RESET_VAL: u32 = 0x2000_003f;

    pub const TLB_FLUSH_ALL: u32 = 0;
    pub const PTC_FLUSH_ALL: u32 = 0;
}

assert_eq_size!(mc; MemoryController, [u8; 0x240]);

impl MemoryController {}

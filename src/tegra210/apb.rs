use register::{mmio::*, register_bitfields};

#[allow(non_snake_case)]
#[repr(C)]
pub struct MiscPP {
    reserved0: [u8; 0x8],                // 0x0
    pub STRAPPING_OPT_A: ReadWrite<u32>, // 0x8
    reserved1: [u8; 0x18],               // 0xC
    pub CONFIG_CTL: ReadWrite<u32>,      // 0x24
    reserved2: [u8; 0x18],               // 0x28
    pub PINMUX_GLOBAL: ReadWrite<u32>,   // 0x40
    reserved3: [u8; 0x3bc],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct MiscGP {
    reserved0: [u8; 0x4],                    // 0x0
    HIDREV: ReadOnly<u32>,                   // 0x4
    reserved1: [u8; 0x8],                    // 0x8
    ASDBGREG: ReadWrite<u32>,                // 0x10
    reserved2: [u8; 0xc0],                   // 0x14
    SDMMC1_CLK_LPBK_CONTROL: ReadWrite<u32>, // 0xd4
    SDMMC3_CLK_LPBK_CONTROL: ReadWrite<u32>, // 0xd8
    EMMC2_PAD_CFG_CONTROL: ReadWrite<u32>,   //  0xdc
    EMMC4_PAD_CFG_CONTROL: ReadWrite<u32>,   // 0xe0
    ALS_PROX_INT_CFGPADCTRL: ReadWrite<u32>, // 0xe4
    AP_READY_CFGPADCTRL: ReadWrite<u32>,     // 0xe8
    AP_WAKE_BT_CFGPADCTRL: ReadWrite<u32>,   // 0xec
    AP_WAKE_NFC_CFGPADCTRL: ReadWrite<u32>,  // 0xf0
    AUD_MCLK_CFGPADCTRL: ReadWrite<u32>,     // 0xf4
    BATT_BCL_CFGPADCTRL: ReadWrite<u32>,     // 0xf8
    BT_RST_CFGPADCTRL: ReadWrite<u32>,       // 0xfc
    // TODO: finish this
    reserved3: [u8; 0x300],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Misc {
    pub pp: MiscPP,
    reserved0: [u8; 0x400], // SC1X_PADS
    pub gp: MiscGP,
}

assert_eq_size!(pp; MiscPP, [u8; 0x400]);
assert_eq_size!(gp; MiscGP, [u8; 0x400]);

#[allow(non_snake_case)]
#[repr(C)]
pub struct AMBAPeripheralBus {
    pub misc: Misc,
}

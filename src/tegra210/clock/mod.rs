use register::mmio::ReadWrite;

use super::timer;

#[derive(Debug, Clone)]
pub struct Clock {
    reset: u32,
    enable: u32,
    source: u32,
    index: u8,
    clock_source: u32,
    clock_divisor: u32,
}

// Register helper definition
const CLK_RST_CONTROLLER_RST_DEVICES_L: u32 = 0x4;
const CLK_RST_CONTROLLER_RST_DEVICES_H: u32 = 0x8;
const CLK_RST_CONTROLLER_RST_DEVICES_U: u32 = 0xC;
const CLK_RST_CONTROLLER_RST_DEVICES_X: u32 = 0x28C;
const CLK_RST_CONTROLLER_RST_DEVICES_Y: u32 = 0x2A4;
const CLK_RST_CONTROLLER_RST_DEVICES_V: u32 = 0x358;
const CLK_RST_CONTROLLER_RST_DEVICES_W: u32 = 0x35C;

const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_X: u32 = 0x280;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_Y: u32 = 0x298;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_V: u32 = 0x360;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_W: u32 = 0x364;

const CLK_NO_SOURCE: u32 = 0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTA: u32 = 0x178;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTB: u32 = 0x17C;
const CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X: u32 = 0x180;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTC: u32 = 0x1A0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTD: u32 = 0x1C0;
const CLK_RST_CONTROLLER_CLK_SOURCE_TSEC: u32 = 0x1F4;
const CLK_RST_CONTROLLER_CLK_SOURCE_SOR1: u32 = 0x410;
const CLK_RST_CONTROLLER_CLK_SOURCE_SE: u32 = 0x42C;
const CLK_RST_CONTROLLER_CLK_SOURCE_TSECB: u32 = 0x6d8;

const CLOCK_BASE: u32 = 0x6000_6000;

// Clock definition
impl Clock {
    pub const UART_A: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UARTA,
        index: 0x6,
        clock_source: 0,
        clock_divisor: 0,
    };
    pub const UART_B: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UARTB,
        index: 0x7,
        clock_source: 0,
        clock_divisor: 0,
    };
    pub const UART_C: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UARTC,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const HOST1X: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X,
        index: 0x1C,
        clock_source: 4,
        clock_divisor: 3,
    };

    pub const SE: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SE,
        index: 0x1F,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const TSEC: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSEC,
        index: 0x13,
        clock_source: 0,
        clock_divisor: 2,
    };

    pub const TSECB: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSECB,
        index: 0xE,
        clock_source: 0,
        clock_divisor: 2,
    };

    pub const SOR_SAFE: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divisor: 0,
    };
    
    pub const DPAUX: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: 0x15,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const SOR0: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: 0x16,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const SOR1: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SOR1,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 2,
    };

    pub const DPAUX1: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: 0xF,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const MIPI_CAL: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x18,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const CSI: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x14,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const DSI: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x10,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const DSIB: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_NO_SOURCE,
        index: 0x12,
        clock_source: 0,
        clock_divisor: 0,
    };

    pub const KFUSE: Clock = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x8,
        clock_source: 0,
        clock_divisor: 0,
    };
}

impl Clock {
    pub fn set_reset(&self, set_reset: bool) {
        let reset_reg = unsafe { &(*((CLOCK_BASE + self.reset) as *const ReadWrite<u32>)) };

        let current_value = reset_reg.get();
        let mask = (1 << self.index & 0x1f);
        let value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        reset_reg.set(value);
    }

    pub fn set_enable(&self, set_enable: bool) {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };

        let current_value = enable_reg.get();
        let mask = (1 << (self.index & 0x1f));
        let value = if set_enable {
            current_value | mask
        } else {
            current_value & !mask
        };

        enable_reg.set(value);
    }

    pub fn is_enabled(&self) -> bool {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };

        let current_value = enable_reg.get();
        let mask = (1 << (self.index & 0x1f));

        (current_value & mask) == mask
    }

    pub fn enable(&self) {
        // Disable clock
        self.disable();

        // Setup clock source if needed
        if self.source != CLK_NO_SOURCE {
            let source_reg = unsafe { &(*((CLOCK_BASE + self.source) as *const ReadWrite<u32>)) };
            source_reg.set(self.clock_divisor | (self.clock_source << 29));
        }

        // Enable clock
        self.set_enable(true);
        self.set_reset(false);
        //assert!(self.is_enabled());
    }

    pub fn disable(&self) {
        self.set_reset(true);
        self.set_enable(false);
        //assert!(!self.is_enabled());
    }
}

use register::mmio::ReadWrite;

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

const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;

const CLK_RST_CONTROLLER_CLK_SOURCE_UARTA: u32 = 0x178;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTB: u32 = 0x17C;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTC: u32 = 0x1A0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UARTD: u32 = 0x1C0;

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
}

impl Clock {
    pub fn set_reset(&self, set_reset: bool) {
        let reset_reg = unsafe { &(*((CLOCK_BASE + self.reset) as *const ReadWrite<u32>)) };

        let current_value = reset_reg.get();
        let mask = (1 << self.index) & 0x1f;
        let value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        reset_reg.set(value);
    }

    pub fn set_enable(&self, set_reset: bool) {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };

        let current_value = enable_reg.get();
        let mask = (1 << self.index) & 0x1f;
        let value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        enable_reg.set(value);
    }

    pub fn enable(&self) {
        // Disable clock
        self.disable();

        // Setup clock source if needed
        if self.source != 0 {
            let source_reg = unsafe { &(*((CLOCK_BASE + self.source) as *const ReadWrite<u32>)) };
            source_reg.set(self.clock_divisor | (self.clock_source << 29));
        }

        // Enable clock
        self.set_enable(true);
        self.set_reset(false);
    }

    pub fn disable(&self) {
        self.set_reset(true);
        self.set_enable(false);
    }
}

use enum_primitive::FromPrimitive;
use register::mmio::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GpioPort {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    AA,
    BB,
    CC,
    DD,
    EE,
    FF,
}
enum_from_primitive! {
    pub enum GpioMode {
        GPIO = 0,
        SFIO = 1,
    }
}

enum_from_primitive! {
    pub enum GpioDirection {
        Output = 0,
        Input = 1,
    }
}

pub enum GpioLevel {
    Low = 0,
    High = 1,
}

#[derive(Copy, Clone)]
pub enum GpioConfig {
    Input,
    OutputLow,
    OutputHigh,
}

pub struct Gpio(pub GpioPort, pub usize);

impl Gpio {
    fn as_bank(&self) -> usize {
        (((self.0 as usize) * 8) + self.1) >> 5
    }

    fn read_flag(&self, reg: &ReadWrite<u32>) -> u32 {
        (reg.get() >> self.1) & 1
    }

    pub fn get_mode(&self) -> GpioMode {
        let control = GpioCtlr::get();
        let config_reg = unsafe {
            &(*control).banks[self.as_bank()].gpio_config
                [((self.0 as usize) & (GPIO_PORT_COUNT - 1))]
        };

        GpioMode::from_u32(self.read_flag(config_reg)).unwrap()
    }

    pub fn set_mode(&self, mode: GpioMode) {
        let control = GpioCtlr::get();
        let config_reg = unsafe {
            &(*control).banks[self.as_bank()].gpio_config
                [((self.0 as usize) & (GPIO_PORT_COUNT - 1))]
        };
        let mut config = config_reg.get();

        match mode {
            GpioMode::GPIO => {
                config |= 1 << self.1;
            }
            GpioMode::SFIO => {
                config &= !(1 << self.1);
            }
        }

        config_reg.set(config);
    }

    pub fn get_direction(&self) -> GpioDirection {
        let control = GpioCtlr::get();
        let direction_reg = unsafe {
            &(*control).banks[self.as_bank()].gpio_direction_out
                [((self.0 as usize) & (GPIO_PORT_COUNT - 1))]
        };

        GpioDirection::from_u32(self.read_flag(direction_reg)).unwrap()
    }

    pub fn set_direction(&self, direction: GpioDirection) {
        let control = GpioCtlr::get();
        let direction_reg = unsafe {
            &(*control).banks[self.as_bank()].gpio_direction_out
                [((self.0 as usize) & (GPIO_PORT_COUNT - 1))]
        };
        let mut direction_value = direction_reg.get();

        match direction {
            GpioDirection::Output => {
                direction_value |= 1 << self.1;
            }
            GpioDirection::Input => {
                direction_value &= !(1 << self.1);
            }
        }

        direction_reg.set(direction_value);
    }

    pub fn set_level(&self, level: GpioLevel) {
        let control = GpioCtlr::get();
        let level_reg = unsafe {
            &(*control).banks[self.as_bank()].gpio_out[((self.0 as usize) & (GPIO_PORT_COUNT - 1))]
        };
        let mut level_value = level_reg.get();

        match level {
            GpioLevel::High => {
                level_value |= 1 << self.1;
            }
            GpioLevel::Low => {
                level_value &= !(1 << self.1);
            }
        }

        level_reg.set(level_value);
    }

    pub fn config(&self, config: GpioConfig) {
        self.set_mode(GpioMode::GPIO);

        match config {
            GpioConfig::Input => {
                self.set_direction(GpioDirection::Input);
            }
            GpioConfig::OutputLow => {
                self.set_direction(GpioDirection::Output);
                self.set_level(GpioLevel::Low);
            }
            GpioConfig::OutputHigh => {
                self.set_direction(GpioDirection::Output);
                self.set_level(GpioLevel::High);
            }
        }
    }
}

pub const GPIO_PORT_COUNT: usize = 4;
pub const GPIO_BANKS_COUNT: usize = 8;

#[allow(non_snake_case)]
#[repr(C)]
pub struct GpioCtl {
    gpio_config: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_direction_out: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_out: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_in: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_int_status: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_int_enable: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_int_level: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_int_clear: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_config: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_dir_out: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_out: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_in: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_int_status: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_int_enable: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_int_level: [ReadWrite<u32>; GPIO_PORT_COUNT],
    gpio_masked_int_clear: [ReadWrite<u32>; GPIO_PORT_COUNT],
}

impl GpioCtlr {
    pub fn get() -> *const GpioCtlr {
        0x6000_D000 as *const GpioCtlr
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct GpioCtlr {
    banks: [GpioCtl; 8],
}

assert_eq_size!(pp; GpioCtlr, [u8; 0x800]);

//pub fn get_config(gpio:)

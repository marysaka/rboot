use crate::tegra210::uart::{UARTRegister, UART};
use core::fmt::Write;
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

pub enum Type {
    A,
    B,
    C,
}

struct UARTLogger {
    level: Level,
    uart_type: Type,
}

impl UARTLogger {
    fn get_uart(&self) -> UART {
        match self.uart_type {
            Type::A => UART::A,
            Type::B => UART::B,
            Type::C => UART::C,
        }
    }

    fn set_type(&mut self, uart_type: Type) {
        self.uart_type = uart_type;
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}

impl log::Log for UARTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut uart = self.get_uart();
            writeln!(&mut uart, "{} - {}\r", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static mut LOGGER: UARTLogger = UARTLogger {
    uart_type: Type::A,
    level: Level::Info,
};

pub fn init(uart_type: Type, level: Level) -> Result<(), SetLoggerError> {
    unsafe {
        LOGGER.set_type(uart_type);
        LOGGER.set_level(level);
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

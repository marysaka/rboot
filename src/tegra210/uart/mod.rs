use crate::tegra210::clock::Clock;
use crate::tegra210::timer::usleep;
use core::ptr::NonNull;
use register::mmio::ReadWrite;

#[allow(non_snake_case)]
#[repr(C)]
pub struct UARTRegister {
    THR_DLAB: ReadWrite<u32>,
    IER_DLAB: ReadWrite<u32>,
    IIR_FCR: ReadWrite<u32>,
    LCR: ReadWrite<u32>,
    MCR: ReadWrite<u32>,
    LSR: ReadWrite<u32>,
    MSR: ReadWrite<u32>,
    SPR: ReadWrite<u32>,
    IRDA_CSR: ReadWrite<u32>,
    RX_FIFO_CFG: ReadWrite<u32>,
    MIE: ReadWrite<u32>,
    VENDOR_STATUS: ReadWrite<u32>,
    unk: [u8; 0xC],
    ASR: ReadWrite<u32>,
}

unsafe impl core::marker::Sync for UART {}

pub struct UART {
    pub register_base: *const UARTRegister,
    pub clock: &'static Clock,
}

pub const LSR_RDR: u32 = 0x1;
pub const LSR_THRE: u32 = 0x20;
pub const LSR_TMTY: u32 = 0x40;

impl UART {
    pub const A: Self = UART {
        register_base: 0x7000_6000 as *const UARTRegister,
        clock: &Clock::UART_A,
    };
    pub const B: Self = UART {
        register_base: 0x7000_6040 as *const UARTRegister,
        clock: &Clock::UART_B,
    };
    pub const C: Self = UART {
        register_base: 0x7000_6200 as *const UARTRegister,
        clock: &Clock::UART_C,
    };

    // TODO: setup clocks for them
    //pub const D: Self = UART { register_base: 0x70006300, clock: Clock::UART_D };
    //pub const E: Self = UART { register_base: 0x70006400, clock: Clock::UART_E };

    pub fn init(&self, baud: u32) {
        self.clock.enable();

        // wait for idle state
        self.wait_idle(LSR_TMTY);

        let rate = (8 * baud + 408_000_000) / (16 * baud);

        let uart_base = unsafe { &(*self.register_base) };

        // disable interrupts
        uart_base.IER_DLAB.set(0);

        // No hardware flow control
        uart_base.MCR.set(0);

        // DLAB + WORD_LENGTH_8
        uart_base.LCR.set(0x83);

        uart_base.THR_DLAB.set(rate);
        uart_base.IER_DLAB.set(rate >> 8);

        // WORD_LENGTH_8
        uart_base.LCR.set(0x3);

        // FIFO mode (16550 mode) + Clear TX + Clear RX
        uart_base.IIR_FCR.set(0x7);

        // FIXME: why is this hanging if I don't do that? compiler bug?
        let tmp = (baud + 999_999) / baud;
        usleep(3 * tmp);

        // wait until ready
        self.wait_idle(LSR_TMTY);
        self.wait_receive();
    }

    pub fn wait_idle(&self, val: u32) {
        let lsr_reg = unsafe { &((*self.register_base).LSR) };

        while (lsr_reg.get() & val) == 0 {}
    }

    pub fn wait_transmit(&self) {
        let lsr_reg = unsafe { &((*self.register_base).LSR) };

        while (lsr_reg.get() & LSR_TMTY) == 0 {}
    }

    pub fn wait_receive(&self) {
        let lsr_reg = unsafe { &((*self.register_base).LSR) };

        while (lsr_reg.get() & LSR_RDR) != 0 {}
    }

    pub fn put_char(&self, c: u8) {
        self.write_data(&[c])
    }

    pub fn write_data(&self, data: &[u8]) {
        self.wait_transmit();

        let thr_reg = unsafe { &((*self.register_base).THR_DLAB) };

        for val in data {
            thr_reg.set(u32::from(*val));
        }
    }

    pub fn get_char(&self) -> u8 {
        self.wait_receive();

        let receive_reg = unsafe { &((*self.register_base).THR_DLAB) };
        receive_reg.get() as u8
    }

    pub fn put_u32(&self, d: u32) {
        let mut digits: [u8; 10] = [0x0; 10];
        let mut d = d;

        for i in digits.iter_mut() {
            *i = ((d % 10) + 0x30) as u8;

            d /= 10;

            if d == 0 {
                break;
            }
        }

        for c in digits.iter().rev() {
            self.put_char(*c);
        }
    }
}

impl core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_data(s.as_bytes());

        // Wait for everything to be written
        self.wait_transmit();
        Ok(())
    }
}

use register::mmio::ReadWrite;

pub struct UART {
    register_base: u64,
}

impl UART {
    pub fn get_a() -> UART {
        UART {
            register_base: 0x70006000,
        }
    }

    pub fn get_b() -> UART {
        UART {
            register_base: 0x70006040,
        }
    }

    pub fn get_c() -> UART {
        UART {
            register_base: 0x70006200,
        }
    }

    pub fn get_d() -> UART {
        UART {
            register_base: 0x70006300,
        }
    }

    pub fn get_e() -> UART {
        UART {
            register_base: 0x70006400,
        }
    }

    pub fn wait_transmit(&self) {
        let lsr_reg = unsafe { &(*((self.register_base + 0x14) as *const ReadWrite<u8>)) };

        while (lsr_reg.get() & 0x40) == 0 {}
    }

    pub fn wait_receive(&self) {
        let lsr_reg = unsafe { &(*((self.register_base + 0x14) as *const ReadWrite<u8>)) };

        while (lsr_reg.get() & 1) == 0 {}
    }

    pub fn put_char(&self, c: u8) {
        self.wait_transmit();

        let thr_reg = unsafe { &(*((self.register_base) as *const ReadWrite<u8>)) };
        thr_reg.set(c);
    }

    pub fn get_char(&self) -> u8 {
        self.wait_receive();

        let receive_reg = unsafe { &(*((self.register_base) as *const ReadWrite<u8>)) };
        receive_reg.get()
    }
}

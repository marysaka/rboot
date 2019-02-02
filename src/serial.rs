use crate::tegra210::uart::UART;
use embedded_serial::ImmutBlockingTx;

pub enum SerialError {
    Unknown
}

impl ImmutBlockingTx for UART {
    type Error = SerialError;

    fn putc(&self, ch: u8) -> Result<(), Self::Error> {

        self.put_char(ch);
        self.wait_transmit();
        Ok(())
    }
}
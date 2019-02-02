use register::mmio::*;

pub fn usleep(micro_seconds: u32) {
    let tmr_us = unsafe { &(*(0x6000_5010 as *const ReadWrite<u32>)) };

    let start_time = tmr_us.get();

    while (tmr_us.get() - start_time) <= micro_seconds {}
}

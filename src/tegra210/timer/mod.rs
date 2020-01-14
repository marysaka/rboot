use register::mmio::*;

pub fn usleep(micro_seconds: u32) {
    let tmr_us = unsafe { &(*(0x6000_5010 as *const ReadWrite<u32>)) };

    let start_time = tmr_us.get();

    while (tmr_us.get() - start_time) <= micro_seconds {}
}

pub fn get_ms() -> u32 {
    let rtc_shadow_sec = unsafe { &(*(0x7000_E00C as *const ReadWrite<u32>)) };
    let rtc_ms = unsafe { &(*(0x7000_E010 as *const ReadWrite<u32>)) };

    rtc_ms.get() | (rtc_shadow_sec.get() << 10)
}

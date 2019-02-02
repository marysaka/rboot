#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use core::ptr;

use cortex_a::{asm, regs::*};

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;

            f()
        }
    };
}

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    /*use crate::uart::UART;
    use core::fmt::Write;

    let mut uart = UART::A;
    uart.init(115_200);
    writeln!(&mut uart, "panic occurred: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());*/

    unsafe {
        reboot_to_rcm();
    };
    loop {}
}

extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
    static _stack_bottom: u8;
    static _stack_top: u8;
}

#[no_mangle]
pub unsafe extern "C" fn reboot_to_rcm() {
    asm!(
        "mov x1, xzr
    mov w2, #0x2
    movz x1, 0xE450
    movk x1, #0x7000, lsl 16
    str w2, [x1]
    movz x1, #0xE400
    movk x1, #0x7000, lsl 16
    ldr w0, [x1]
    orr w0, w0, #0x10
    str w0, [x1]"
    );
}

#[link_section = ".crt0"]
//#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // FIXME: move this later when we will want relocation
    SP.set(&_stack_top as *const u8 as u64);
    _start_with_stack()
}

#[no_mangle]
pub unsafe extern "C" fn _start_with_stack() -> ! {
    // Clean .bss
    // FIXME: Will not work when we will want relocation
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    // Call user entry point
    extern "Rust" {
        fn main() -> ();
    }

    main();
    reboot_to_rcm();

    loop {}
}

#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use core::ptr;

use crate::exception_vectors;

use crate::tegra210;
use core::fmt::Write;

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

#[link_section = ".text.boot"]
//#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Switch to EL1 from EL2 and setup the stack
    /*asm!(
        "msr sctlr_el1, xzr
         mrs x0, hcr_el2
         orr x0, x0, #(1 << 31)
         msr hcr_el2, x0
         mov x0, #0b00101
         msr spsr_el2, x0
         msr elr_el2, $0
         msr sp_el1, $1
         eret"
        ::
        "r"(_start_with_stack as *const () as u64),
        "r"(&_stack_top as *const u8 as usize)
        ::
        "volatile"
    );*/

    asm!("mov sp, $0
          mrs x0, mpidr_el1
          and x0, x0, #3
          b _start_with_stack"
         :: "r"(&_stack_top as *const u8 as usize) :: "volatile");
    core::intrinsics::unreachable()
}

#[no_mangle]
pub unsafe extern "C" fn _start_with_stack(core_id: u32) -> ! {
    // I don't trust the payload before us so we make sure to power off
    if core_id != 0 {
        loop { asm!("wfe"); }
    }

    exception_vectors::set_vbar_el2();

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

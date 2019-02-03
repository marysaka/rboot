use crate::tegra210::uart::UART;
use core::fmt::Write;

use crate::rt;

global_asm!(
    "
    .macro  vector_entry  label
    .align  7
    b       \\label
    .endm

    .macro  vector_not_handled
    .align  7
    b       _unhandled_vector
    .endm

    .section .vectors, \"ax\"
    .align  11
    .global el1_vector_table;
    el1_vector_table:
        /* Current EL with SP0 */
        vector_entry _start_with_stack
        vector_not_handled
        vector_not_handled
        vector_not_handled

        .align 9
        /* Current EL with SPx */
        vector_entry _current_elx_sync
        vector_not_handled
        vector_not_handled
        vector_not_handled

        .align 9
        /* EL0 exception to EL1 (AArch64) */
        vector_not_handled
        vector_not_handled
        vector_not_handled
        vector_not_handled

        .align 9
        /* EL0 exception to EL1 (AArch32) */
        vector_not_handled
        vector_not_handled
        vector_not_handled
        vector_not_handled
    "
);

pub unsafe fn set_vbar_el1() {
    extern "C" {
        static el1_vector_table: u64;
    }

    let exception_vectors_start: u64 = &el1_vector_table as *const _ as u64;
    asm!("msr vbar_el1, $0" :: "r"(exception_vectors_start) :: "volatile");
}

global_asm!("
    .macro  push, xreg1, xreg2
        stp     \\xreg1, \\xreg2, [sp, #-16]!
    .endm

    .macro  pop, xreg1, xreg2
        ldp     \\xreg1, \\xreg2, [sp], #16
    .endm

    .macro __save_registers
        push    x29, x30
        push    x27, x28
        push    x25, x26
        push    x23, x24
        push    x21, x22
        push    x19, x20
        push    x17, x18
        push    x15, x16
        push    x13, x14
        push    x11, x12
        push    x9,  x10
        push    x7,  x8
        push    x5,  x6
        push    x3,  x4
        push    x1,  x2

        mrs     x20, esr_el1
        push    x20, x0

        mrs     x0, elr_el1
        mrs     x1, spsr_el1
        push    x0, x1

        mrs     x0, far_el1
        push    x0, x0
    .endm
    .macro __restore_registers
        pop    x0, x0

        pop    x0, x1
        msr    elr_el1, x0
        msr    spsr_el1, x1

        pop    x20, x0

        pop    x1,  x2
        pop    x3,  x4
        pop    x5,  x6
        pop    x7,  x8
        pop    x9,  x10
        pop    x11, x12
        pop    x13, x14
        pop    x15, x16
        pop    x17, x18
        pop    x19, x20
        pop    x21, x22
        pop    x23, x24
        pop    x25, x26
        pop    x27, x28
        pop    x29, x30
    .endm
    ");

#[naked]
#[no_mangle]
unsafe fn _unhandled_vector() {
    asm!("__save_registers
          mov x0, sp
          bl unhandled_vector
          __restore_registers
          eret");
}

#[naked]
#[no_mangle]
unsafe fn _current_elx_sync() {
    asm!("__save_registers
          mov x0, sp
          bl current_elx_sync
          __restore_registers
          eret");
}

#[repr(C)]
struct ExceptionInfo {
    far_duplicate: u64,
    far: u64,
    pc: u64,
    cpsr: u64,
    esr: u64,
    x: [u64; 31],
}

unsafe fn dump_exception(exception: &mut ExceptionInfo) {
    let mut uart_a = UART::A;


    write!(&mut uart_a, "Fault address:\t{:20x}\r\n", exception.far);
    write!(&mut uart_a, "Register dump:\r\n");
    write!(&mut uart_a, "PC:\t{:20x}\t", exception.pc);
    write!(&mut uart_a, "CPSR:\t{:20x}\t", exception.cpsr);
    write!(&mut uart_a, "ESR:\t{:20x}\r\n", exception.esr);

    for (index, value) in exception.x.iter_mut().enumerate()
    {
        // "X{}:"
        // We do taht because {} is what is making us crashing, but {:x} is fine so we use it!
        uart_a.put_char(88);
        uart_a.put_u64(index as u64);
        write!(&mut uart_a, ":\t{:20x}\t", *value);

        if (index % 3) == 0 {
            write!(&mut uart_a, "\r\n");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn unhandled_vector(exception: &mut ExceptionInfo) {
    let mut uart_a = UART::A;
    write!(&mut uart_a, "\r\n");
    write!(&mut uart_a, "Unhandled vector!\r\n");

    dump_exception(exception);

    rt::reboot_to_rcm();
}

pub fn get_exception_type_el1(esr: u64) -> &'static str {
    let exception_class = esr >> 26;

    match exception_class {
        0x18 => "Configurable trap",
        0x22 => "PC alignment exception",
        0x25 => "Data abort",
        0x26 => "Stack alignment exception",
        0x30 => "Debug exception",
        _ => "Unknown exception"
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_sync(exception: &mut ExceptionInfo) {
    let mut uart_a = UART::A;
    write!(&mut uart_a, "\r\n");
    write!(&mut uart_a, "Sync ELX Exception (");
    uart_a.write_data(get_exception_type_el1(exception.esr).as_bytes());
    write!(&mut uart_a, ")\r\n");
    dump_exception(exception);

    rt::reboot_to_rcm();
}
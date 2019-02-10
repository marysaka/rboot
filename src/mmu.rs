#![allow(clippy::identity_op)]

use register::register_bitfields;

use crate::tegra210::uart::UART;
use core::fmt::Write;
use core::ops::{BitAnd, Not};
use num_traits::Num;

extern "C" {
    static mut __start_text__: u8;
    static mut __end_text__: u8;
    static mut __start_ro__: u8;
    static mut __end_ro__: u8;
    static mut __start_data__: u8;
    static mut __end_data__: u8;
    static mut __start_bss__: u8;
    static mut __end_bss__: u8;
    static _stack_bottom: u8;
    static _stack_top: u8;
}

const PAGE_GRANULE: usize = 12; // 4K

const ENTRY_SHIFT: usize = 3;
const ENTRIES_PER_LEVEL_BITS: usize = PAGE_GRANULE - ENTRY_SHIFT;
const ENTRIES_PER_LEVEL: usize = 1 << ENTRIES_PER_LEVEL_BITS;
const NUM_ENTRIES_4KIB: usize = ENTRIES_PER_LEVEL;

const L3_INDEX_LSB: usize = PAGE_GRANULE;
const L2_INDEX_LSB: usize = L3_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;
const L1_INDEX_LSB: usize = L2_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;
const L0_INDEX_LSB: usize = L1_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;

// 33 bits address space
const TARGET_BITS: usize = 33;

const LVL2_ENTRY_SIZE: usize = 1 << (TARGET_BITS - L2_INDEX_LSB);
const LVL3_ENTRY_SIZE: usize = 1 << (TARGET_BITS - L3_INDEX_LSB);

const NUM_LVL1_ENTRIES: usize = 1 << (TARGET_BITS - L1_INDEX_LSB);
const NUM_LVL2_ENTRIES: usize = LVL2_ENTRY_SIZE / 512;
const NUM_LVL3_ENTRIES: usize = LVL3_ENTRY_SIZE / 8;

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL3 {
    entries: [u64; NUM_ENTRIES_4KIB],
}

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL2 {
    entries: [u64; NUM_ENTRIES_4KIB],
    lvl3: [TableLVL3; NUM_LVL1_ENTRIES],
}

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL1 {
    entries: [u64; NUM_ENTRIES_4KIB],
    lvl2: [TableLVL2; NUM_LVL1_ENTRIES],
}

static mut LVL1_TABLE: TableLVL1 = TableLVL1 {
    entries: [0x0; /*NUM_LVL1_ENTRIES*/ NUM_ENTRIES_4KIB],
    lvl2: [TableLVL2 {
        entries: [0x0; NUM_ENTRIES_4KIB],
        lvl3: [TableLVL3 {
            entries: [0x0; NUM_ENTRIES_4KIB],
        }; NUM_LVL1_ENTRIES],
    }; NUM_LVL1_ENTRIES],
};

pub enum MemoryPermission {
    R,
    W,
    X,
    RW,
    RX,
    RWX,
}

register_bitfields! {u64,
    STAGE1_NEXTLEVEL_DESCRIPTOR [
        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        TYPE OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        ADDRESS OFFSET(12) NUMBITS(36) [],

        PXN OFFSET(59) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        XN OFFSET(60) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        AP_TABLE OFFSET(61) NUMBITS(2) [
            NO_EFFECT = 0b00,
            NO_EL0 = 0b01,
            NO_WRITE = 0b10,
            NO_WRITE_EL0_READ = 0b11
        ],

        NS OFFSET(63) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

register_bitfields! {u64,
    STAGE2_BLOCK_DESCRIPTOR [
        VALID OFFSET(0) NUMBITS(1) [
            True = 1
        ],

        TYPE OFFSET(1) NUMBITS(1) [
            Block = 0
        ],

        MEMORY_ATTR OFFSET(2) NUMBITS(4) [],

        AP OFFSET(6) NUMBITS(2) [
            RW_CURRENT_EL = 0b00,
            RW_BOTH_EL = 0b01,
            RO_CURRENT_EL = 0b10,
            RO_BOTH_EL = 0b11
        ],

        SH OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        AF OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        ADDRESS OFFSET(21) NUMBITS(27) [],

        CONTIGUOUS OFFSET(52) NUMBITS(1) [],

        XN OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]

}

register_bitfields! {u64,
    STAGE2_NEXTLEVEL_DESCRIPTOR [
        VALID OFFSET(0) NUMBITS(1) [
            True = 1
        ],

        TYPE OFFSET(1) NUMBITS(1) [
            Table = 1
        ],

        ADDRESS OFFSET(12) NUMBITS(36) []
    ]
}

register_bitfields! {u64,
    STAGE3_TABLE_DESCRIPTOR [
        VALID OFFSET(0) NUMBITS(1) [
            True = 1
        ],

        TYPE OFFSET(1) NUMBITS(1) [
            Table = 1
        ],

        MEMORY_ATTR OFFSET(2) NUMBITS(4) [],

        AP OFFSET(6) NUMBITS(2) [
            RW_CURRENT_EL = 0b00,
            RW_BOTH_EL = 0b01,
            RO_CURRENT_EL = 0b10,
            RO_BOTH_EL = 0b11
        ],

        SH OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        AF OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        ADDRESS OFFSET(12) NUMBITS(36) [],

        XN OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

mod mem_attr {
    // Device-nGnRnE (strongly ordered)
    pub const MMIO: u64 = 0;

    // outer: writeback/alloc, inner: writeback/alloc
    pub const NORMAL: u64 = 1;

    // outer: non-cacheable, inner: non-cacheable
    pub const NORMAL_UNCACHED: u64 = 2;
}

pub fn align_up<T: Num + Not<Output = T> + BitAnd<Output = T> + Copy>(addr: T, align: T) -> T {
    align_down(addr + (align - T::one()), align)
}

pub fn align_down<T: Num + Not<Output = T> + BitAnd<Output = T> + Copy>(addr: T, align: T) -> T {
    addr & !(align - T::one())
}

fn create_lvl2_block_entry(vaddr: u64, paddr: u64, memory_attribute: u64) {
    let lvl1_align_size = 1 << L1_INDEX_LSB;
    let lvl2_align_size = 1 << L2_INDEX_LSB;

    let lvl1_index = (vaddr / lvl1_align_size) as usize % NUM_LVL1_ENTRIES;
    let lvl2_index = (vaddr / lvl2_align_size) as usize % NUM_ENTRIES_4KIB;

    let flags = STAGE2_BLOCK_DESCRIPTOR::VALID::True
        + STAGE2_BLOCK_DESCRIPTOR::TYPE::Block
        + STAGE2_BLOCK_DESCRIPTOR::MEMORY_ATTR.val(memory_attribute)
        + STAGE2_BLOCK_DESCRIPTOR::AF::True
        + STAGE2_BLOCK_DESCRIPTOR::SH::InnerShareable;
    unsafe {
        LVL1_TABLE.lvl2[lvl1_index].entries[lvl2_index] =
            (flags + STAGE2_BLOCK_DESCRIPTOR::ADDRESS.val(paddr >> L2_INDEX_LSB)).value;
    };
}

fn create_lvl2_table_entry(vaddr: u64, table_address: u64) {
    let mut uart_a = &mut UART::A;
    let lvl1_align_size = 1 << L1_INDEX_LSB;
    let lvl2_align_size = 1 << L2_INDEX_LSB;

    let lvl1_index = (vaddr / lvl1_align_size) as usize % NUM_LVL1_ENTRIES;
    let lvl2_index = (vaddr / lvl2_align_size) as usize % NUM_ENTRIES_4KIB;

    writeln!(&mut uart_a, "lvl1_index: 0x{:x}\r", lvl1_index).unwrap();
    writeln!(&mut uart_a, "lvl2_index: 0x{:x}\r", lvl2_index).unwrap();

    let flags = STAGE2_NEXTLEVEL_DESCRIPTOR::VALID::True + STAGE2_NEXTLEVEL_DESCRIPTOR::TYPE::Table;
    unsafe {
        LVL1_TABLE.lvl2[lvl1_index].entries[lvl2_index] =
            (flags + STAGE2_NEXTLEVEL_DESCRIPTOR::ADDRESS.val(table_address >> L3_INDEX_LSB)).value;
    };
}

fn map_lvl2_block(vaddr: u64, paddr: u64, size: u64, memory_attribute: u64) {
    let lvl2_align_size = 1 << L2_INDEX_LSB;
    let size = align_up(size, lvl2_align_size);

    let mut vaddr = align_down(vaddr, lvl2_align_size);
    let mut paddr = align_down(paddr, lvl2_align_size);
    let mut page_count = size / lvl2_align_size;

    let mut uart_a = &mut UART::A;
    writeln!(&mut uart_a, "page_count: 0x{:x}\r", page_count).unwrap();

    while page_count != 0 {
        create_lvl2_block_entry(vaddr, paddr, memory_attribute);
        vaddr += lvl2_align_size;
        paddr += lvl2_align_size;
        page_count -= 1;
    }
}

pub unsafe fn init_page_mapping() {
    let common_flags = STAGE1_NEXTLEVEL_DESCRIPTOR::VALID::True
        + STAGE1_NEXTLEVEL_DESCRIPTOR::TYPE::Table
        + STAGE1_NEXTLEVEL_DESCRIPTOR::NS::True;

    // Setup LVL1 entries
    for (lvl1_index, lvl1_entry) in LVL1_TABLE
        .entries
        .iter_mut()
        .enumerate()
        .take(NUM_LVL1_ENTRIES)
    {
        let address = &LVL1_TABLE.lvl2[lvl1_index].entries[0] as *const _ as u64 >> 12;
        *lvl1_entry = (common_flags + STAGE1_NEXTLEVEL_DESCRIPTOR::ADDRESS.val(address)).value;
    }

    // We setup one page of 2MB, we don't need more.
    // TODO: fix LVL3 to map by page of 4KB
    let text_start = &__start_text__ as *const _ as u64;

    let base_addr = align_down(text_start, 1 << L2_INDEX_LSB);
    map_lvl2_block(base_addr, base_addr, 1 << L2_INDEX_LSB, mem_attr::NORMAL);

    // map some MMIOs
    const MMIO_RANGE_SIZE: u64 = 0x200000;
    const MMIO_RANGE_0_ADDR: u64 = 0x50000000;
    const MMIO_RANGE_1_ADDR: u64 = 0x60000000;
    const MMIO_RANGE_2_ADDR: u64 = 0x70000000;

    map_lvl2_block(
        MMIO_RANGE_0_ADDR,
        MMIO_RANGE_0_ADDR,
        MMIO_RANGE_SIZE,
        mem_attr::MMIO,
    );
    map_lvl2_block(
        MMIO_RANGE_1_ADDR,
        MMIO_RANGE_1_ADDR,
        MMIO_RANGE_SIZE,
        mem_attr::MMIO,
    );
    map_lvl2_block(
        MMIO_RANGE_2_ADDR,
        MMIO_RANGE_2_ADDR,
        MMIO_RANGE_SIZE,
        mem_attr::MMIO,
    );
}

pub unsafe fn setup() {
    init_page_mapping();

    // start MMU state setup

    asm!("dsb sy" :::: "volatile");

    // setup MAIR
    let mair: u64 = (0x00 << (mem_attr::MMIO * 8))
        | (0xFF << (mem_attr::NORMAL * 8))
        | (0x44 << (mem_attr::NORMAL_UNCACHED * 8));
    asm!("msr mair_el1, $0" :: "r"(mair) :: "volatile");

    // setup TTBR0/TTBR1
    asm!("msr ttbr0_el1, $0" :: "r"(&LVL1_TABLE.entries[0] as *const _ as u64) :: "volatile");

    // TODO: virt mapping
    //asm!("msr ttbr1_el1, $0" :: "r"(&LVL1_TABLE.entries[0] as *const _ as u64) :: "volatile");

    asm!("tlbi vmalle1" :::: "volatile");
    asm!("ic iallu" :::: "volatile");
    asm!("dsb sy" :::: "volatile");
    asm!("isb" :::: "volatile");

    // TODO: register field for this
    // TCR_PS_40BIT | TCR_TG0_4K | MMU_MEMORY_TCR_OUTER_RGN0(MMU_MEMORY_RGN_WRITE_BACK_ALLOCATE) | MMU_MEMORY_TCR_INNER_RGN0(MMU_MEMORY_RGN_WRITE_BACK_ALLOCATE) | MMU_MEMORY_TCR_T0SZ(MONBITS))
    let tcr: u64 = (2 << 16) | (0 << 14) | (1 << 10) | (1 << 8) | ((64 - TARGET_BITS as u64) << 0);
    asm!("msr tcr_el1, $0":: "r"(tcr) :: "volatile");
    asm!("isb");

    let mut cpu_ectrl: u64;

    asm!("mrs $0, S3_1_C15_C2_1" : "=r"(cpu_ectrl) ::: "volatile");
    cpu_ectrl |= 1 << 6;
    asm!("msr S3_1_C15_C2_1, $0" :: "r"(cpu_ectrl) :: "volatile");

    // finally enable MMU and cache
    let mut ctrl: u64;
    asm!("mrs $0, sctlr_el1" : "=r"(ctrl) ::: "volatile");

    ctrl |= 0xC00800; // mandatory reserved bits
    ctrl |= (1 << 12) |    // I, Instruction cache enable. This is an enable bit for instruction caches at EL0 and EL1
            (1 << 4)  |    // SA0, tack Alignment Check Enable for EL0
            (1 << 3)  |    // SA, Stack Alignment Check Enable
            (1 << 2)  |    // C, Data cache enable. This is an enable bit for data caches at EL0 and EL1
            (1 << 1)  |    // A, Alignment check enable bit
            (1 << 0); // set M, enable MMU

    asm!("msr sctlr_el1, $0" :: "r"(ctrl) :: "volatile");

    // and hope that it's okayish
    asm!("dsb sy");
    asm!("isb");
}

pub unsafe fn map_normal_page(vaddr: u64, paddr: u64, len: u64, permission: MemoryPermission) {
    if len == 0 {
        return;
    }

    let mut uart_a = &mut UART::A;

    let lvl2_align_size = 1 << L2_INDEX_LSB;
    let page_align_size = 1 << L3_INDEX_LSB;

    let paddr = align_down(paddr, page_align_size);
    let vaddr_page_align = align_down(vaddr, page_align_size);
    let vaddr_page_align_end = align_up(vaddr_page_align + len, page_align_size);

    let raw_lvl2_index = (align_down(vaddr, lvl2_align_size) / lvl2_align_size) as usize;
    let lvl1_index = (raw_lvl2_index / NUM_ENTRIES_4KIB) % NUM_LVL1_ENTRIES;

    let raw_lvl3_index = (vaddr_page_align / page_align_size) as usize;
    let lvl2_index = (raw_lvl3_index / NUM_ENTRIES_4KIB) % NUM_ENTRIES_4KIB;
    let lvl3_index_start = (raw_lvl3_index % NUM_ENTRIES_4KIB) % NUM_ENTRIES_4KIB;
    let lvl3_index_end =
        (((vaddr_page_align_end / page_align_size) as usize) % NUM_ENTRIES_4KIB) % NUM_ENTRIES_4KIB;

    writeln!(&mut uart_a, "vaddr :0x{:x}\r", vaddr).unwrap();
    writeln!(&mut uart_a, "size :0x{:x}\r", len).unwrap();
    writeln!(
        &mut uart_a,
        "size (aligned): 0x{:x}\r",
        (lvl3_index_end - lvl3_index_start) * 4096
    )
    .unwrap();
    writeln!(&mut uart_a, "LVL1[0x{:x}]\r", lvl1_index).unwrap();
    writeln!(&mut uart_a, "LVL2[0x{:x}]\r", lvl2_index).unwrap();
    writeln!(&mut uart_a, "LVL3_START[0x{:x}]\r", lvl3_index_start).unwrap();
    writeln!(&mut uart_a, "LVL3_END[0x{:x}]\r", lvl3_index_end).unwrap();

    let mut flags = STAGE3_TABLE_DESCRIPTOR::VALID::True
        + STAGE3_TABLE_DESCRIPTOR::TYPE::Table
        + STAGE3_TABLE_DESCRIPTOR::MEMORY_ATTR.val(mem_attr::NORMAL)
        + STAGE3_TABLE_DESCRIPTOR::SH::InnerShareable
        + STAGE3_TABLE_DESCRIPTOR::AF::True;

    flags = match permission {
        MemoryPermission::R => {
            flags + STAGE3_TABLE_DESCRIPTOR::AP::RO_CURRENT_EL + STAGE3_TABLE_DESCRIPTOR::XN::True
        }
        MemoryPermission::RW | MemoryPermission::W => {
            flags + STAGE3_TABLE_DESCRIPTOR::AP::RW_CURRENT_EL + STAGE3_TABLE_DESCRIPTOR::XN::True
        }
        MemoryPermission::RWX => {
            flags + STAGE3_TABLE_DESCRIPTOR::AP::RW_CURRENT_EL + STAGE3_TABLE_DESCRIPTOR::XN::False
        }
        MemoryPermission::RX | MemoryPermission::X => {
            flags + STAGE3_TABLE_DESCRIPTOR::AP::RO_CURRENT_EL + STAGE3_TABLE_DESCRIPTOR::XN::False
        }
    };

    let mut x = 0;
    for i in lvl3_index_start..lvl3_index_end {
        writeln!(&mut uart_a, "LVL3[0x{:x}]\r", i).unwrap();
        let addr = (paddr + (x * page_align_size)) >> 12;
        writeln!(&mut uart_a, "0x{:x}\r", addr).unwrap();
        LVL1_TABLE.lvl2[lvl1_index].lvl3[lvl2_index].entries[i] =
            (flags + STAGE3_TABLE_DESCRIPTOR::ADDRESS.val(addr)).value;
        x += 1;
    }
}

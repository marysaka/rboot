#![allow(clippy::identity_op)]

use crate::utils;
use register::register_bitfields;

extern "C" {
    static mut __text_start__: u8;
    static mut __text_end__: u8;
    static mut __vectors_start__: u8;
    static mut __vectors_end__: u8;
    static mut __rodata_start__: u8;
    static mut __rodata_end__: u8;
    static mut __data_start__: u8;
    static mut __data_end__: u8;
    static mut __bss_start__: u8;
    static mut __bss_end__: u8;
    static _stack_bottom: u8;
    static _stack_top: u8;
}

const PAGE_GRANULE_4K: usize = 12;
const PAGE_GRANULE_16K: usize = 14;
const PAGE_GRANULE_64K: usize = 16;

const PAGE_GRANULE: usize = PAGE_GRANULE_4K;

const ENTRY_SHIFT: usize = 3;
const ENTRIES_PER_LEVEL_BITS: usize = PAGE_GRANULE - ENTRY_SHIFT;
const ENTRIES_PER_LEVEL: usize = 1 << ENTRIES_PER_LEVEL_BITS;

const L3_INDEX_LSB: usize = PAGE_GRANULE;
const L2_INDEX_LSB: usize = L3_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;
const L1_INDEX_LSB: usize = L2_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;
const L0_INDEX_LSB: usize = L1_INDEX_LSB + ENTRIES_PER_LEVEL_BITS;

// 33 bits address space
const TARGET_BITS: usize = 33;

const LVL2_ENTRY_SIZE: usize = 1 << (TARGET_BITS - L2_INDEX_LSB);
const LVL3_ENTRY_SIZE: usize = 1 << (TARGET_BITS - L3_INDEX_LSB);

const NUM_LVL1_ENTRIES: usize = 1 << (TARGET_BITS - L1_INDEX_LSB);
const NUM_LVL2_ENTRIES: usize = LVL2_ENTRY_SIZE / ENTRIES_PER_LEVEL;
const NUM_LVL3_ENTRIES: usize = LVL3_ENTRY_SIZE / NUM_LVL1_ENTRIES;

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL3 {
    entries: [u64; ENTRIES_PER_LEVEL],
}

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL2 {
    entries: [u64; ENTRIES_PER_LEVEL],
    lvl3: [TableLVL3; ENTRIES_PER_LEVEL],
}

#[repr(C)]
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct TableLVL1 {
    entries: [u64; NUM_LVL1_ENTRIES],
    lvl2: [TableLVL2; NUM_LVL1_ENTRIES],
}

static mut LVL1_TABLE: TableLVL1 = TableLVL1 {
    entries: [0x0; NUM_LVL1_ENTRIES],
    lvl2: [TableLVL2 {
        entries: [0x0; ENTRIES_PER_LEVEL],
        lvl3: [TableLVL3 {
            entries: [0x0; ENTRIES_PER_LEVEL],
        }; ENTRIES_PER_LEVEL],
    }; NUM_LVL1_ENTRIES],
};

#[derive(Copy, Clone, PartialEq)]
pub enum MemoryPermission {
    Invalid,
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
            True = 1
        ],

        TYPE OFFSET(1) NUMBITS(1) [
            Table = 1
        ],

        ADDRESS_4K OFFSET(12) NUMBITS(36) [],
        ADDRESS_16K OFFSET(14) NUMBITS(34) [],
        ADDRESS_64K OFFSET(16) NUMBITS(32) [],

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

        ADDRESS_4K OFFSET(21) NUMBITS(27) [],
        ADDRESS_16K OFFSET(25) NUMBITS(23) [],
        ADDRESS_64K OFFSET(29) NUMBITS(19) [],

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

        ADDRESS_4K OFFSET(12) NUMBITS(36) [],
        ADDRESS_16K OFFSET(14) NUMBITS(34) [],
        ADDRESS_64K OFFSET(16) NUMBITS(32) []
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

pub mod mem_attr {
    // Device-nGnRnE (strongly ordered)
    pub const MMIO: u64 = 0;

    // outer: writeback/alloc, inner: writeback/alloc
    pub const NORMAL: u64 = 1;

    // outer: non-cacheable, inner: non-cacheable
    pub const NORMAL_UNCACHED: u64 = 2;
}

fn create_lvl2_block_entry(vaddr: u64, paddr: u64, memory_attribute: u64) {
    let lvl1_align_size = 1 << L1_INDEX_LSB;
    let lvl2_align_size = 1 << L2_INDEX_LSB;

    let lvl1_index = (vaddr / lvl1_align_size) as usize % NUM_LVL1_ENTRIES;
    let lvl2_index = (vaddr / lvl2_align_size) as usize % ENTRIES_PER_LEVEL;

    let flags = STAGE2_BLOCK_DESCRIPTOR::VALID::True
        + STAGE2_BLOCK_DESCRIPTOR::TYPE::Block
        + STAGE2_BLOCK_DESCRIPTOR::MEMORY_ATTR.val(memory_attribute)
        + STAGE2_BLOCK_DESCRIPTOR::AF::True
        + STAGE2_BLOCK_DESCRIPTOR::SH::InnerShareable;
    unsafe {
        LVL1_TABLE.lvl2[lvl1_index].entries[lvl2_index] =
            (flags + STAGE2_BLOCK_DESCRIPTOR::ADDRESS_4K.val(paddr >> L2_INDEX_LSB)).value;
        asm!("dsb sy" ::: "memory");
    };
}

fn create_lvl2_table_entry(vaddr: u64, table_address: u64) {
    let lvl1_align_size = 1 << L1_INDEX_LSB;
    let lvl2_align_size = 1 << L2_INDEX_LSB;

    let lvl1_index = (vaddr / lvl1_align_size) as usize % NUM_LVL1_ENTRIES;
    let lvl2_index = (vaddr / lvl2_align_size) as usize % ENTRIES_PER_LEVEL;

    let flags = STAGE2_NEXTLEVEL_DESCRIPTOR::VALID::True + STAGE2_NEXTLEVEL_DESCRIPTOR::TYPE::Table;
    unsafe {
        LVL1_TABLE.lvl2[lvl1_index].entries[lvl2_index] = (flags
            + STAGE2_NEXTLEVEL_DESCRIPTOR::ADDRESS_4K.val(table_address >> L3_INDEX_LSB))
        .value;
        asm!("dsb sy" ::: "memory");
    };
}

fn create_lvl3_page(vaddr: u64, paddr: u64, permission: MemoryPermission, memory_attribute: u64) {
    let lvl1_align_size = 1 << L1_INDEX_LSB;
    let lvl2_align_size = 1 << L2_INDEX_LSB;
    let lvl3_align_size = 1 << L3_INDEX_LSB;

    let lvl1_index = (vaddr / lvl1_align_size) as usize % NUM_LVL1_ENTRIES;
    let lvl2_index = (vaddr / lvl2_align_size) as usize % ENTRIES_PER_LEVEL;
    let lvl3_index = (vaddr / lvl3_align_size) as usize % ENTRIES_PER_LEVEL;

    // LVL2 entry is missing, add one
    unsafe {
        if LVL1_TABLE.lvl2[lvl1_index].entries[lvl2_index] == 0 {
            create_lvl2_table_entry(
                vaddr,
                &LVL1_TABLE.lvl2[lvl1_index].lvl3[lvl2_index].entries[0] as *const _ as u64,
            );
        }
    }

    let mut flags = STAGE3_TABLE_DESCRIPTOR::VALID::True
        + STAGE3_TABLE_DESCRIPTOR::TYPE::Table
        + STAGE3_TABLE_DESCRIPTOR::MEMORY_ATTR.val(memory_attribute)
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
        _ => flags,
    };

    unsafe {
        let value = if permission == MemoryPermission::Invalid {
            0
        } else {
            (flags + STAGE3_TABLE_DESCRIPTOR::ADDRESS.val(paddr >> PAGE_GRANULE)).value
        };

        LVL1_TABLE.lvl2[lvl1_index].lvl3[lvl2_index].entries[lvl3_index] = value;
        asm!("dsb sy" ::: "memory");
    };
}

pub fn map_normal_page(vaddr: u64, paddr: u64, size: u64, permission: MemoryPermission) {
    map_page(vaddr, paddr, size, permission, mem_attr::NORMAL)
}

pub fn map_page(
    vaddr: u64,
    paddr: u64,
    size: u64,
    permission: MemoryPermission,
    memory_attribute: u64,
) {
    if size == 0 {
        return;
    }

    let lvl3_align_size = 1 << L3_INDEX_LSB;
    let size = utils::align_up(size, lvl3_align_size);

    let mut vaddr = utils::align_down(vaddr, lvl3_align_size);
    let mut paddr = utils::align_down(paddr, lvl3_align_size);
    let mut page_count = size / lvl3_align_size;

    while page_count != 0 {
        create_lvl3_page(vaddr, paddr, permission, memory_attribute);
        vaddr += lvl3_align_size;
        paddr += lvl3_align_size;
        page_count -= 1;
    }

    // TLB maintenance
    // TODO: EL2 & EL3
    unsafe {
        asm!("dsb sy" :::: "volatile");
        asm!("isb" :::: "volatile");
    };

    invalidate_tlb_all()
}

pub fn unmap_page(vaddr: u64, size: u64) {
    if size == 0 {
        return;
    }

    let lvl3_align_size = 1 << L3_INDEX_LSB;
    let size = utils::align_up(size, lvl3_align_size);

    let mut vaddr = utils::align_down(vaddr, lvl3_align_size);
    let mut page_count = size / lvl3_align_size;

    while page_count != 0 {
        create_lvl3_page(vaddr, 0, MemoryPermission::Invalid, mem_attr::NORMAL);
        vaddr += lvl3_align_size;
        page_count -= 1;
    }

    // TLB maintenance
    // TODO: EL2 & EL3
    unsafe {
        asm!("dsb sy" :::: "volatile");
        asm!("isb" :::: "volatile");
    };

    invalidate_tlb_all()
}

fn map_lvl2_block(vaddr: u64, paddr: u64, size: u64, memory_attribute: u64) {
    let lvl2_align_size = 1 << L2_INDEX_LSB;
    let size = utils::align_up(size, lvl2_align_size);

    let mut vaddr = utils::align_down(vaddr, lvl2_align_size);
    let mut paddr = utils::align_down(paddr, lvl2_align_size);
    let mut page_count = size / lvl2_align_size;

    while page_count != 0 {
        create_lvl2_block_entry(vaddr, paddr, memory_attribute);
        vaddr += lvl2_align_size;
        paddr += lvl2_align_size;
        page_count -= 1;
    }
}

unsafe fn init_executable_mapping() {
    let text_start = &__text_start__ as *const _ as u64;
    let text_end = &__text_end__ as *const _ as u64;
    map_normal_page(
        text_start,
        text_start,
        text_end - text_start,
        MemoryPermission::RX,
    );

    let ro_start = &__rodata_start__ as *const _ as u64;
    let ro_end = &__rodata_end__ as *const _ as u64;
    map_normal_page(ro_start, ro_start, ro_end - ro_start, MemoryPermission::R);

    let data_start = &__data_start__ as *const _ as u64;
    let data_end = &__data_end__ as *const _ as u64;
    map_normal_page(
        data_start,
        data_start,
        data_end - data_start,
        MemoryPermission::RW,
    );

    let bss_start = &__bss_start__ as *const _ as u64;
    let bss_end = &__bss_end__ as *const _ as u64;
    map_normal_page(
        bss_start,
        bss_start,
        bss_end - bss_start,
        MemoryPermission::RW,
    );

    // Setup our stack
    let stack_start = &_stack_bottom as *const _ as u64;
    let stack_end = &_stack_top as *const _ as u64;
    map_normal_page(
        stack_start,
        stack_start,
        stack_end - stack_start,
        MemoryPermission::RW,
    );

    // Also setup the exception vector
    let vectors_start = &__vectors_start__ as *const _ as u64;
    let vectors_end = &__vectors_end__ as *const _ as u64;
    map_normal_page(
        vectors_start,
        vectors_start,
        vectors_end - vectors_start,
        MemoryPermission::RX,
    );
}

fn init_page_mapping() {
    let common_flags = STAGE1_NEXTLEVEL_DESCRIPTOR::VALID::True
        + STAGE1_NEXTLEVEL_DESCRIPTOR::TYPE::Table
        + STAGE1_NEXTLEVEL_DESCRIPTOR::NS::False;

    // Setup LVL1 entries
    unsafe {
        for (lvl1_index, lvl1_entry) in LVL1_TABLE
            .entries
            .iter_mut()
            .enumerate()
            .take(NUM_LVL1_ENTRIES)
        {
            let address =
                &LVL1_TABLE.lvl2[lvl1_index].entries[0] as *const _ as u64 >> PAGE_GRANULE;
            *lvl1_entry =
                (common_flags + STAGE1_NEXTLEVEL_DESCRIPTOR::ADDRESS_4K.val(address)).value;
        }
        init_executable_mapping();
    }

    // map some MMIOs
    const MMIO_RANGE_SIZE: u64 = 0x200000;
    const MMIO_RANGE_0_ADDR: u64 = 0x50000000;
    const MMIO_RANGE_1_ADDR: u64 = 0x60000000;
    const MMIO_RANGE_2_ADDR: u64 = 0x70000000;

    const MMIO_RANGE_3_ADDR: u64 = 0x54400000;
    const MMIO_RANGE_4_ADDR: u64 = 0x54100000;

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
    map_lvl2_block(
        MMIO_RANGE_3_ADDR,
        MMIO_RANGE_3_ADDR,
        MMIO_RANGE_SIZE,
        mem_attr::MMIO,
    );
    map_lvl2_block(
        MMIO_RANGE_4_ADDR,
        MMIO_RANGE_4_ADDR,
        MMIO_RANGE_SIZE,
        mem_attr::MMIO,
    );
}

fn get_sctlr() -> u64 {
    let mut ctrl: u64;

    unsafe {
        match utils::get_current_el() {
            1 => asm!("mrs $0, sctlr_el1" : "=r"(ctrl) ::: "volatile"),
            2 => asm!("mrs $0, sctlr_el2" : "=r"(ctrl) ::: "volatile"),
            3 => asm!("mrs $0, sctlr_el3" : "=r"(ctrl) ::: "volatile"),
            _ => unimplemented!(),
        }
    }

    ctrl
}

fn set_sctlr(new_sctlr: u64) {
    unsafe {
        match utils::get_current_el() {
            1 => asm!("msr sctlr_el1, $0" :: "r"(new_sctlr) :: "volatile"),
            2 => asm!("msr sctlr_el2, $0" :: "r"(new_sctlr) :: "volatile"),
            3 => asm!("msr sctlr_el3, $0" :: "r"(new_sctlr) :: "volatile"),
            _ => unimplemented!(),
        }

        asm!("isb" :::: "volatile");
    }
}

pub fn switch_ttbr(address: u64) {
    let current_el = utils::get_current_el();

    let sctlr = get_sctlr();

    let mut temp_sctlr = sctlr;

    // Disbale MMU and caches.
    temp_sctlr &= !((1 << 0) | (1 << 2) | (1 << 12));

    set_sctlr(temp_sctlr);

    // Invalidate TLB
    invalidate_tlb_all();

    unsafe {
        // Set TTRBR
        match current_el {
            1 => asm!("msr ttbr0_el1, $0" :: "r"(address) :: "volatile"),
            2 => asm!("msr ttbr0_el2, $0" :: "r"(address) :: "volatile"),
            3 => asm!("msr ttbr0_el3, $0" :: "r"(address) :: "volatile"),
            _ => unimplemented!(),
        }

        asm!("isb" :::: "volatile");
    }

    // Restore previous sctlr.
    set_sctlr(sctlr);
}

pub fn invalidate_tlb_all() {
    unsafe {
        match utils::get_current_el() {
            1 => asm!("tlbi vmalle1" :::: "volatile"),
            2 => asm!("tlbi alle2" :::: "volatile"),
            3 => asm!("tlbi alle3" :::: "volatile"),
            _ => unimplemented!(),
        }

        asm!("dsb sy" :::: "volatile");
        asm!("isb" :::: "volatile");
    }
}

pub fn invalidate_icache_all() {
    unsafe {
        asm!("ic iallu" :::: "volatile");
        asm!("dsb sy" :::: "volatile");
        asm!("isb" :::: "volatile");
    }
}

pub fn enable_icache() {
    invalidate_icache_all();
    set_sctlr(get_sctlr() | (1 << 12));
}

pub fn disable_icache() {
    set_sctlr(get_sctlr() | !(1 << 12));
}

pub fn is_icache_enabled() -> bool {
    (get_sctlr() & (1 << 12)) != 0
}

/// Enable receiving instruction cache/tbl maintenance operations.
fn enable_maintenance_operations() {
    if utils::get_current_el() == 3 {
        unsafe {
            let mut cpu_ectrl: u64;
            asm!("mrs $0, S3_1_C15_C2_1" : "=r"(cpu_ectrl) ::: "volatile");
            cpu_ectrl |= 1 << 6;
            asm!("msr S3_1_C15_C2_1, $0" :: "r"(cpu_ectrl) :: "volatile");
        }
    }
}

unsafe fn set_mair_ttbr_tcr(mair: u64, ttbr: u64, tcr: u64) {
    asm!("dsb sy" :::: "volatile");

    match utils::get_current_el() {
        1 => {
            asm!("msr mair_el1, $0" :: "r"(mair) :: "volatile");
            asm!("msr ttbr0_el1, $0" :: "r"(ttbr) :: "volatile");
            asm!("msr tcr_el1, $0":: "r"(tcr) :: "volatile");
        }
        2 => {
            asm!("msr mair_el2, $0" :: "r"(mair) :: "volatile");
            asm!("msr ttbr0_el2, $0" :: "r"(ttbr) :: "volatile");
            asm!("msr tcr_el2, $0":: "r"(tcr) :: "volatile");
        }
        3 => {
            asm!("msr mair_el3, $0" :: "r"(mair) :: "volatile");
            asm!("msr ttbr0_el3, $0" :: "r"(ttbr) :: "volatile");
            asm!("msr tcr_el3, $0":: "r"(tcr) :: "volatile");
        }
        _ => unimplemented!(),
    }

    asm!("isb");
}

pub unsafe fn setup() {
    init_page_mapping();

    // Make sure TLB/cache operations are activated.
    enable_maintenance_operations();

    // Setup memory attributes, lvl1 table and tcr.

    let mair: u64 = (0x00 << (mem_attr::MMIO * 8))
        | (0xFF << (mem_attr::NORMAL * 8))
        | (0x44 << (mem_attr::NORMAL_UNCACHED * 8));

    let ttbr = &LVL1_TABLE.entries[0] as *const _ as u64;

    // TODO: register field for this
    // TCR_PS_40BIT | TCR_TG0_4K | MMU_MEMORY_TCR_OUTER_RGN0(MMU_MEMORY_RGN_WRITE_BACK_ALLOCATE) | MMU_MEMORY_TCR_INNER_RGN0(MMU_MEMORY_RGN_WRITE_BACK_ALLOCATE) | MMU_MEMORY_TCR_T0SZ(MONBITS))
    let tcr: u64 = (2 << 16) | (0 << 14) | (1 << 10) | (1 << 8) | ((64 - TARGET_BITS as u64) << 0);
    set_mair_ttbr_tcr(mair, ttbr, tcr);

    // Invalidate icache as we are going to activate it.
    invalidate_icache_all();

    // finally enable MMU and cache
    let mut ctrl = get_sctlr();

    ctrl |= 0xC00800; // mandatory reserved bits
    ctrl |= (1 << 12) |    // I, Instruction cache enable. This is an enable bit for instruction caches at EL0 and EL1
            (1 << 4)  |    // SA0, Stack Alignment Check Enable for EL0
            (1 << 3)  |    // SA, Stack Alignment Check Enable
            (1 << 2)  |    // C, Data cache enable. This is an enable bit for data caches at EL0 and EL1
            (1 << 1)  |    // A, Alignment check enable bit
            (1 << 0); // set M, enable MMU

    set_sctlr(ctrl);

    // and hope that it's okayish
    asm!("dsb sy");
    asm!("isb");
}

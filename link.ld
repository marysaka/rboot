OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
ENTRY(_start)

MEMORY
{
	CBOOT_FAUX_KERNEL : ORIGIN = 0x80080000, LENGTH = 256K
}

SECTIONS
{
	. = ORIGIN(CBOOT_FAUX_KERNEL);
    .text ALIGN(4K) :
	{
		*(.crt0)
		*(.text)
	} >CBOOT_FAUX_KERNEL

	.rodata ALIGN(4K) : { *(.rodata) } >CBOOT_FAUX_KERNEL
	.data ALIGN(4K) : { *(.data) } >CBOOT_FAUX_KERNEL

	.bss ALIGN(4K) :
	{
		_sbss = .;
		*(.bss COMMON)
		_ebss = .;
	} >CBOOT_FAUX_KERNEL

	. = ALIGN(8);

	. = . + 0x1000; /* 4kB of stack memory */

    _stack_top = .;

	/DISCARD/ : {
		*(.comment*)
		*(.eh_frame*)
		*(.gcc_except_table*)
		*(.note*)
		*(.rel.eh_frame*)
	}
}
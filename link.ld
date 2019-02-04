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
	.text ALIGN(1K) :
	{
		KEEP(*(.text.boot))
		*(.text .text.*)
	} >CBOOT_FAUX_KERNEL

	.vectors ALIGN(2K): { *(.vectors) } >CBOOT_FAUX_KERNEL

	.rodata ALIGN(1K) : { *(.rodata) } >CBOOT_FAUX_KERNEL
	.data ALIGN(1K) : { *(.data) } >CBOOT_FAUX_KERNEL

	.bss ALIGN(1K) :
	{
		_sbss = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(8);
		_ebss = .;
	} >CBOOT_FAUX_KERNEL

	.stack ALIGN(1K) :
	{
		. = ALIGN(16);
		_stack_bottom = .;
		. = . + 0x1000; /* 4kB of stack memory */
		. = ALIGN(16);
		PROVIDE(_stack_top = .);
	} >CBOOT_FAUX_KERNEL

	/DISCARD/ : {
		*(.comment*)
		*(.eh_frame*)
		*(.gcc_except_table*)
		*(.note*)
		*(.rel.eh_frame*)
	}
}
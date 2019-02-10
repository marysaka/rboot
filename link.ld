OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
ENTRY(_start)

MEMORY
{
	CBOOT_FAUX_KERNEL : ORIGIN = 0x80080000, LENGTH = 256M
}

SECTIONS
{
	. = ORIGIN(CBOOT_FAUX_KERNEL);
	.text ALIGN(4K) : {
		__start_text__ = .;

		KEEP(*(.text.boot))
		*(.text .text.*)

		__end_text__ = .;
	} >CBOOT_FAUX_KERNEL

	.vectors ALIGN(4K): {
		__start_vectors__ = .;
		*(.vectors .vectors.*)
		__end_vectors__ = .;
	} >CBOOT_FAUX_KERNEL

	.rodata ALIGN(4K) : {
		__start_ro__ = .;
		*(.rodata .rodata.*)
		__end_ro__ = .;
	} >CBOOT_FAUX_KERNEL

	.data ALIGN(4K) : {
		__start_data__ = .;
		*(.data .data.*)
		__end_data__ = .;
	} >CBOOT_FAUX_KERNEL

	.bss ALIGN(4K) : {
		__start_bss__ = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(8);
		__end_bss__ = .;
	} >CBOOT_FAUX_KERNEL

	.stack ALIGN(4K) : {
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
NAME = rboot
TARGET = aarch64-thog-none

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld

.PHONY: all clippy clean objdump nm

all: $(NAME).bin

re: clean all

target/$(TARGET)/debug/$(NAME): $(SOURCES)
	cargo xbuild --target=$(TARGET)

$(NAME).bin: target/$(TARGET)/debug/$(NAME)
	cp $< .
	#cargo objcopy -- -O binary $< $(NAME).bin
	aarch64-none-elf-objcopy -O binary $< $(NAME).bin
clean:
	cargo clean

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex $(NAME)

nm:
	cargo nm --target $(TARGET) -- $(NAME) | sort

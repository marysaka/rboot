NAME = rboot
TARGET = aarch64-thog-none

SOURCES = $(shell find src -name '*.rs') link.ld

.PHONY: all clippy clean objdump

all: $(NAME).bin

re: clean all

target/$(TARGET)/debug/$(NAME): $(SOURCES)
	@RUST_TARGET_PATH=$(shell pwd) cargo xbuild --target=$(TARGET)

$(NAME).bin: target/$(TARGET)/debug/$(NAME)
	cp $< .
	cargo objcopy -- -O binary $< $(NAME).bin

clean:
	rm -f rboot rboot.bin
	cargo clean

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex $(NAME)

clippy:
	@RUST_TARGET_PATH=$(shell pwd) cargo xclippy --target $(TARGET)
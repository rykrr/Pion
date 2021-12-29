################################################################################################################################################################

TOOLCHAIN	= aarch64-none-elf
TARGET		= aarch64-unknown-none

################################################################################

CC		= $(TOOLCHAIN)-gcc
CCARGS	= -ggdb -ffreestanding -Wall -Wextra -nostdlib -O2 -Wall -Wextra

################################################################################

AS		= $(TOOLCHAIN)-as
GDB		= $(TOOLCHAIN)-gdb
OBJCOPY	= $(TOOLCHAIN)-objcopy
OBJDUMP	= $(TOOLCHAIN)-objdump

################################################################################

PROJECT	= pion
SOURCES	= $(shell find src/ -type f -regex '.*\.r?s')
RELEASE	= target/$(TARGET)/release/$(PROJECT)
DEBUG	= target/$(TARGET)/debug/$(PROJECT)
OUTPUT	= $(DEBUG)

################################################################################

KERNEL	= kernel.img
SYMBOLS	= kernel.sym
SERIAL	= serial.log

################################################################################

ifdef DISPLAY
override DISPLAY := DISPLAY=:$(DISPLAY)
endif

QEMU		= $(DISPLAY) qemu-system-aarch64
QEMU_ARGS	= -M raspi3b -m 1G -display sdl -kernel $(KERNEL)

################################################################################

.PHONY: all clean init run debug gdb
all: $(KERNEL) $(SYMBOLS)

clean:
	rm -rf target $(KERNEL) $(SYMBOLS) $(SERIAL)

################################################################################

$(KERNEL) $(SYMBOLS): $(SOURCES)
	cargo xbuild --target=$(TARGET).json
	$(OBJCOPY) --only-keep-debug $(OUTPUT) $(SYMBOLS)
	$(OBJCOPY) --strip-all -O binary $(OUTPUT) $(KERNEL)

debug: $(KERNEL)
	$(QEMU) $(QEMU_ARGS) -S -gdb tcp::1337 -serial file:$(SERIAL) -monitor stdio

run: $(KERNEL)
	$(QEMU) $(QEMU_ARGS) -serial stdio

gdb: $(SYMBOLS)
	LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib \
	$(GDB) $(SYMBOLS) \
		-ex 'dir src' \
		-ex 'target remote localhost:1337'

init:
	rustup default nightly
	rustup target add $(TARGET)

################################################################################

# --- Cấu hình -------------------------
TARGET	:= riscv64gc-unknown-none-elf
KERNEL	:= target/$(TARGET)/debug/xv6-rust
KERNEL_R	:= target/$(TARGET)/release/xv6-rust

QEMU	:= qemu-system-riscv64
QEMU_ARGS	:= \
	-machine virt \
	-bios default \
	-nographic \
	-m 128M

GDB	:= riscv64-unknown-elf-gdb

# --- Target chính -----------------------------
PHONY	:= all build run debug gdb disasm clean release

## Biên dịch debug
all:	build

build:
	cargo build --target $(TARGET)

## Chạy kernel với qemu
run:	build
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL)

## Biên dịch release
release:	build
	cargo build --release --target $(TARGET)
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_R)

## debug với qemu
debug:	build
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL) -s -S

## debug với gdb
gdb:	build
	$(GDB) \
		-ex "file $(KERNEL)" \
		-ex "set arch riscv:rv64" \
		-ex "target remote :1234" \
		-ex "break kmain" \
		-ex "continue"

## xem assembly code của kernel
disasm:	build
	cargo objdump -- \
	--disassemble \
	--no-show-raw-insn \
	$(KERNEL) | less

## xem kích thước các section của kernel
size:
	cargo size -- -A $(KERNEL)

## Xoá tất cả build artifacts
clean:
	cargo clean
OUT=target/riscv32imac-unknown-none-elf/release

target := riscv32-os
bbl_path := $(abspath riscv-pk)
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin
usr_path := usr

export SFSIMG = $(usr_path)/rcore32.img

.PHONY: all clean run build asm qemu kernel

# all: kernel
all: $(OUT)/rcore $(OUT)/rcore-bare-metal

# Build the main rcore binary. Relies on an SBI inteface for some functionality.
$(OUT)/rcore: src/*.rs src/*.S Cargo.toml src/s-mode_linker.ld rustup-target
	cargo rustc --release --target riscv32imac-unknown-none-elf --bin rcore \
	    $(GUEST_KERNEL_FEATURE) -- -C link-arg=-Tsrc/s-mode_linker.ld

# Flattened version of rvirt binary.
$(OUT)/rcore.bin: $(OUT)/rcore
	objcopy -S -I elf32-little -O binary --change-addresses -0x80000000 \
	    --set-section-flags .bss=alloc,load,contents \
	    $(OUT)/rcore $(OUT)/rcore.bin

# Build a free standing binary that can run directly on bare metal without any
# SBI provider.
$(OUT)/rcore-bare-metal: $(OUT)/rcore.bin src/*.rs  src/*.S Cargo.toml src/m-mode_linker.ld rustup-target
	PAYLOAD=$(OUT)/rcore.bin cargo  rustc -v --release --target \
	    riscv32imac-unknown-none-elf --bin rcore-bare-metal --features \
	    "physical_symbol_addresses" -- -C link-arg=-Tsrc/m-mode_linker.ld

$(bin): kernel
	mkdir -p target/$(target)/bbl && \
	cd target/$(target)/bbl && \
	$(bbl_path)/configure \
		--with-arch=rv32imac \
		--disable-fp-emulation \
		--host=riscv64-unknown-elf \
		--with-payload=$(abspath $(kernel)) && \
	make -j32 && \
	cp bbl $(abspath $@)

build: $(bin)

run: build qemu

kernel:
	@cargo xbuild --target riscv32-os.json

asm:
#	@riscv64-unknown-elf-objdump -d $(kernel) | less
	@riscv64-unknown-elf-objdump -d $(OUT)/rcore-bare-metal | less

qemu:
#	qemu-system-riscv32 -kernel $(bin) -nographic -machine virt
	qemu-system-riscv32 -machine virt -nographic -smp 1 -kernel $(OUT)/rcore-bare-metal
docker:
	sudo docker run -it --mount type=bind,source=$(shell pwd)/..,destination=/mnt panqinglin/rust_riscv bash

rustup-target:
	rustup target add riscv32imac-unknown-none-elf || true
target := riscv32-os
bbl_path := $(abspath riscv-pk)
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin
usr_path := usr

export SFSIMG = $(usr_path)/rcore32.img

.PHONY: all clean run build asm qemu kernel

all: kernel

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
	@riscv64-unknown-elf-objdump -d $(kernel) | less

qemu:
	qemu-system-riscv32 -kernel $(bin) -nographic -machine virt

docker:
	sudo docker run -it --mount type=bind,source=$(shell pwd)/..,destination=/mnt panqinglin/rust_riscv bash

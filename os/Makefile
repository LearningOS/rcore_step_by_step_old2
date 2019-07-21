target := riscv32-os
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin
usr_path := usr

export SFSIMG = $(abspath $(usr_path)/rcore32.img)

.PHONY: all clean run build asm qemu kernel

all: build

build: $(bin)

run: build qemu

kernel:
	@cargo xbuild --target riscv32-os.json

$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $@

asm:
	@riscv64-unknown-elf-objdump -d $(kernel) | less

qemu: $(bin)
	@qemu-system-riscv32 -nographic -machine virt \
		-kernel opensbi/virt.elf \
		-device loader,file=$(bin),addr=0x80400000

docker:
	sudo docker run -it --mount type=bind,source=$(shell pwd)/..,destination=/mnt panqinglin/rust_riscv bash

OUT=target/riscv32imac-unknown-none-elf/release
kernel := $(OUT)/rbl

.PHONY: build run debug

$(kernel): src/*.rs src/boot/*.rs  src/boot/*.S src/boot/linker.ld
	cargo  rustc --release --target riscv32imac-unknown-none-elf  --bin rbl -- -C link-arg=-Tsrc/boot/linker.ld

build: $(kernel) rustup-target

run: build
	@qemu-system-riscv32 \
		-smp cores=1 \
		-machine virt \
		-kernel $(kernel) \
		-nographic

rustup-target:
	rustup target add riscv32imac-unknown-none-elf || true
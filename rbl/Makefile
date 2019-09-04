kernel := target/riscv32/debug/rbl

.PHONY: build run debug

$(kernel):
	@cargo xbuild --target=riscv32.json

build: $(kernel)

run: build
	@qemu-system-riscv32 \
		-smp cores=1 \
		-machine virt \
		-kernel $(kernel) \
		-nographic \
		-s
debug:
	@gdb $(kernel) -x gdbinit

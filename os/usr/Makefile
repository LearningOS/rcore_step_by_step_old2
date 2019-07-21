out_dir ?= build
out_img ?= rcore32.img

cargo_args := --target riscv32-os.json
rust_src_dir := rust/src/bin
rust_bin_path := rust/target/riscv32-os/debug
rust_bins := $(patsubst $(rust_src_dir)/%.rs, $(rust_bin_path)/%, $(wildcard $(rust_src_dir)/*.rs)) 

.PHONY: all clean rust

all : rust

rust : 
	@echo Building rust user program
	@cd rust && cargo xbuild $(cargo_args)
	@rm -rf $(out_dir)/rust && mkdir -p $(out_dir)/rust
	@echo $(out_dir)
	@echo $(rust_bins)
	@cp -r $(rust_bins) $(out_dir)/rust

clean :
	@rm -rf $(out_dir)
	@cd rust && cargo clean
	@rm -f $(out_img)

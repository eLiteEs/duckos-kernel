# This makefile makes the executable .elf of the kernel

KERNEL = target/x86_64-unknown-none/release/duckos-kernel

.PHONY: all clean test

all: $(KERNEL)

$(KERNEL):
	@echo "Compiling kernel..."
	RUSTFLAGS="-C target-feature=+sse,+sse2" \
		cargo build --release --target x86_64-unknown-none

test:
	cd src/user && rustc +nightly hello.rs \
    		--target x86_64-unknown-none \
    		-C panic=abort \
    		-C opt-level=z \
    		-C link-arg=-Tlinker.ld \
    		-o hello.elf

clean:
	cargo clean


# This makefile makes the executable .elf of the kernel

KERNEL = target/x86_64-unknown-none/release/duckos-kernel

.PHONY: all clean

all: $(KERNEL)

$(KERNEL):
	@echo "Compiling kernel..."
	RUSTFLAGS="-C target-feature=+sse,+sse2" \
		cargo build --release --target x86_64-unknown-none

clean:
	cargo clean


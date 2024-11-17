qemu:
	cargo install cargo-xbuild # bootimage dependency
	cargo install bootimage
	cd iron; cargo +nightly bootimage
	qemu-system-x86_64 -drive format=raw,file=./iron/target/iron/debug/bootimage-iron.bin

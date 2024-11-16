qemu:
	cargo install cargo-xbuild # bootimage dependency
	cargo install bootimage
	cd iron; cargo +nightly bootimage
	qemu-system-x86_64 -drive format=raw,file=.\iron\target\x86_64-unknown-none\debug\bootimage-iron.bin

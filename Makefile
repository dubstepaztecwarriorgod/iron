qemu:
	cargo install bootimage
	cd iron; cargo bootimage	# -> iron/target/iron/debug
	qemu-system-x86_64 -drive format=raw,file=iron/target/iron/debug/iron.bin
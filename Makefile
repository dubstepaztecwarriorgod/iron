qemu:
	# cargo install bootimage
	cd iron; cargo bootimage	# -> iron\target\x86_64-unknown-iron-gnu\
	qemu-system-x86_64 -drive format=raw,file=iron\target\x86_64-unknown-iron-gnu\iron.bin
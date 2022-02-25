# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2022 System76

TARGET = x86_64-unknown-uefi
QEMU = qemu-system-x86_64
QEMU_FLAGS = -accel kvm \
	-M q35 \
	-m 1024 \
	-net none \
	-vga std \
	-bios /usr/share/OVMF/OVMF_CODE.fd

default: target/$(TARGET)/release/uefi-shell.efi

.PHONY: qemu
qemu: target/boot.img
	$(QEMU) $(QEMU_FLAGS) -drive file=$<,format=raw

.PHONY: target/boot.img
target/boot.img: target/efi.img
	dd if=/dev/zero of=$@.tmp bs=1MiB count=50
	parted $@.tmp -s -a minimal mklabel gpt
	parted -s -a minimal $@.tmp mkpart primary fat32 1MiB 49MiB
	parted -s -a minimal $@.tmp set 1 boot on
	dd if=$< of=$@.tmp bs=1MiB count=48 seek=1 conv=notrunc
	mv $@.tmp $@

.PHONY: target/efi.img
target/efi.img: target/$(TARGET)/release/uefi-shell.efi
	dd if=/dev/zero of=$@.tmp bs=1MiB count=48
	mkfs.vfat $@.tmp
	mmd -i $@.tmp efi
	mmd -i $@.tmp efi/boot
	mcopy -i $@.tmp $< ::efi/boot/bootx64.efi
	mv $@.tmp $@

.PHONY: target/$(TARGET)/release/uefi-shell.efi
target/$(TARGET)/release/uefi-shell.efi:
	cargo build \
		--release \
		--target $(TARGET) \
		-Z build-std=core,alloc \
		-Z build-std-features=compiler-builtins-mem

.PHONY: clean
clean:
	cargo clean


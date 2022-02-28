# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2022 System76

TARGET = x86_64-unknown-uefi
OVMF_CODE = /usr/share/OVMF/OVMF_CODE.fd
OVMF_VARS = /usr/share/OVMF/OVMF_VARS.fd
QEMU = qemu-system-x86_64
QEMU_FLAGS = -accel kvm \
	-M q35 \
	-m 1024 \
	-net none \
	-vga std \
	-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE)

default: target/$(TARGET)/release/uefi-shell.efi

.PHONY: qemu
qemu: build/EFI/boot/bootx64.efi
	cp $(OVMF_VARS) target/OVMF_VARS.fd
	$(QEMU) $(QEMU_FLAGS) \
		-drive if=pflash,format=raw,file=target/OVMF_VARS.fd \
		-drive file=fat:rw:build,format=raw

.PHONY: build/EFI/boot/bootx64.efi
build/EFI/boot/bootx64.efi: target/$(TARGET)/release/uefi-shell.efi
	mkdir -p build/EFI/boot
	cp $< $@

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
	rm -rf build

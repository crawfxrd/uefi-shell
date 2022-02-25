# uefi-shell

[![License][License-badge]](./LICENSES/GPL-3.0-only.txt)
[![CI status][CI-badge]](https://github.com/crawfxrd/uefi-shell/actions)

An implementation of the UEFI Shell in Rust.

## Building

Install build dependencies and tools with the provided script:

```sh
./scripts/deps.sh
```

Cargo is used to build the binary.

```sh
cargo build --release --target x86_64-unknown-uefi
```

Make targets are available to generate a disk image file and run it in QEMU.

```sh
make qemu
```

## License

This project is made avilable under the terms of the GNU General Public
License, version 3 only. See [`GPL-3.0-only.txt`](./LICENSES/GPL-3.0-only.txt)
for details.

[CI-badge]: https://github.com/crawfxrd/uefi-shell/workflows/CI/badge.svg
[License-badge]: https://img.shields.io/badge/License-GPL--3.0--only-blue.svg

#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install Rust and dependencies needed for development.

set -eE

# Print bolded message to stdout
msg() {
  echo -e "\x1B[1m$*\x1B[0m"
}

source /etc/os-release

msg ">> Installing system dependencies"
source /etc/os-release
if [[ "${ID}" =~ "arch" ]] || [[ "${ID_LIKE}" =~ "arch" ]]; then
    sudo pacman -S \
        --noconfirm \
        curl \
        make \
        mtools \
        qemu
elif [[ "${ID}" =~ "debian" ]] || [[ "${ID_LIKE}" =~ "debian" ]]; then
    sudo apt update
    sudo apt install \
        --no-install-recommends \
        --yes \
        curl \
        make \
        mtools \
        qemu-system-x86
elif [[ "${ID}" =~ "fedora" ]] || [[ "${ID_LIKE}" =~ "fedora" ]]; then
    sudo dnf install \
        --assumeyes \
        curl \
        make \
        mtools \
        qemu-system-x86
else
    msg "\x1B[31m>> Unknown host:\x1B[0m ${ID}"
    exit 1
fi

if command -v rustup &> /dev/null; then
    msg ">> Updating rustup"
    rustup self update
else
    msg ">> Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain none

    msg ">> Loading Rust environment"
    source "${HOME}/.cargo/env"
fi

msg ">> Installing pinned Rust toolchain and components"
# rustup has no command to specifically install a toolchain from a TOML file.
# Rely on the fact that `show` will install the default toolchain.
rustup show

msg "\x1B[32m>> Successfully installed dependencies\n"

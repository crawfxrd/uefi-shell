// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

use uefi::proto::Protocol;
use uefi::unsafe_guid;

use crate::data_types::ShellFileHandle;

/// Shell application's arguments.
#[repr(C)]
#[unsafe_guid("752F3136-4E16-4FDC-A22A-E5F46812F4CA")]
#[derive(Protocol)]
pub struct ShellParametersProtocol {
    pub argv: *const *const u16,
    pub argc: usize,
    pub stdin: ShellFileHandle,
    pub stdout: ShellFileHandle,
    pub stderr: ShellFileHandle,
}

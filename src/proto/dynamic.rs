// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

use core::ffi::c_void;

use uefi::proto::Protocol;
use uefi::unsafe_guid;

/// Advertise an external shell command.
#[repr(C)]
#[unsafe_guid("3C7200E9-005F-4EA4-87DE-A3DFAC8A27C3")]
#[derive(Protocol)]
pub struct ShellDynamicCommandProtocol {
    command_name: *const u16,
    handler: *const c_void,
    get_help: *const c_void,
}

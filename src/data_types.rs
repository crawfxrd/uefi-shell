// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

use core::ffi::c_void;

use uefi::proto::media::file::FileInfo;
use uefi::Handle;

#[repr(transparent)]
pub struct ShellDeviceNameFlags(u32);

impl ShellDeviceNameFlags {
    pub const USE_COMPONENT_NAME: Self = Self(1);
    pub const USE_DEVICE_PATH: Self = Self(2);
}

// TODO: Move to UEFI lib
#[repr(C)]
#[derive(Debug)]
pub struct ListEntry {
    forward: *mut c_void,
    back: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShellFileInfo {
    link: ListEntry,
    status: uefi::Status,
    full_name: *const u16,
    file_name: *const u16,
    handle: Handle,
    info: FileInfo,
}

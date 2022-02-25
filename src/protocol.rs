// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

use core::ffi::c_void;

use uefi::data_types::{Event, Handle};
use uefi::proto::device_path::DevicePath;
use uefi::proto::Protocol;
use uefi::{unsafe_guid, Guid, Status};

use crate::data_types::{ShellDeviceNameFlags, ShellFileHandle, ShellFileInfo};

/// Shell services for UEFI applications.
#[repr(C)]
#[unsafe_guid("6302D008-7F9B-4F30-87AC-60C9FEF5DA4E")]
#[derive(Protocol)]
#[rustfmt::skip]
pub struct ShellProtocol {
    /// Execute the command line.
    execute: extern "efiapi" fn(
        parent_image_handle: Handle,
        cmd_line: *mut u16,
        env: *mut u16,
        status: *mut Status
    ) -> Status,
    /// Get the environment variable or list of environment variables.
    get_env: extern "efiapi" fn(
        name: *const u16
    ) -> *const u16,
    /// Set a specific environment variable.
    set_env: extern "efiapi" fn(
        name: *const u16,
        value: *const u16,
        volatile: bool
    ) -> Status,
    /// Retrieves a shell command alias.
    get_alias: extern "efiapi" fn(
        alias: *const u16,
        volatile: *mut bool,
    ) -> *const u16,
    /// Changes a shell command alias.
    set_alias: extern "efiapi" fn(
        command: *const u16,
        alias: *const u16,
        replace: bool,
        volatile: bool
    ) -> Status,
    /// Return help information about a specific command.
    get_help_text: extern "efiapi" fn(
        command: *const u16,
        sections: *const u16,
        text: *mut *mut u16,
    ) -> Status,
    /// Get the device path from the mapping.
    get_device_path_from_map: extern "efiapi" fn(
        mapping: *const u16
    ) -> *const DevicePath,
    /// Get one or more mapping entries that most closely match the path.
    get_map_from_device_path: extern "efiapi" fn(
        path: *mut *mut DevicePath
    ) -> *const u16,
    /// Convert a file system-style name to a device path.
    get_device_path_from_file_path: extern "efiapi" fn(
        path: *const u16
    ) -> *mut DevicePath,
    /// Convert a device path to a file system-style path.
    get_file_path_from_device_path: extern "efiapi" fn(
        path: *const DevicePath
    ) -> *mut u16,

    /// Return the current directory on the specified device.
    get_cur_dir: extern "efiapi" fn(
        mapping: *const u16
    ) -> *const u16,
    /// Change the current directory on the specified device.
    set_cur_dir: extern "efiapi" fn(
        filesystem: *const u16,
        dir: *const u16
    ) -> Status,
    /// Open the files that match the specified path.
    open_file_list: extern "efiapi" fn(
        path: *mut u16,
        mode: u64,
        list: *mut *mut ShellFileInfo
    ) -> Status,
    /// Free the file list.
    free_file_list: extern "efiapi" fn(
        list: *mut *mut ShellFileInfo
    ) -> Status,
    /// Deletes the duplicate file names in the given list.
    remove_dup_in_file_list: extern "efiapi" fn(
        list: *mut *mut ShellFileInfo
    ) -> Status,

    /// Determine whether any script files are currently being processed.
    batch_is_active: extern "efiapi" fn() -> bool,
    /// Determine whether the active shell is the root shell.
    is_root_shell: extern "efiapi" fn() -> bool,
    /// Enable page break output mode.
    enable_page_break: extern "efiapi" fn(),
    /// Disable page break output mode.
    disable_page_break: extern "efiapi" fn(),
    /// Determine if page break mode is enabled.
    get_page_break: extern "efiapi" fn() -> bool,
    /// Get the name of the device specified by the device handle.
    get_device_name: extern "efiapi" fn(
        handle: Handle,
        flags: ShellDeviceNameFlags,
        language: *const u8,
        best_name: *const *const u16,
    ) -> Status,

    /// Get the file information from an open file handle.
    get_file_info: extern "efiapi" fn(
        handle: ShellFileHandle
    ) -> *mut ShellFileInfo,
    /// Set the file information to an open file handle.
    set_file_info: extern "efiapi" fn(
        handle: ShellFileHandle,
        info: *const ShellFileInfo
    ) -> Status,
    /// Open a file or directory by file name.
    open_file_by_name: extern "efiapi" fn(
        name: *const u16,
        handle: *mut ShellFileHandle,
        mode: u64
    ) -> Status,
    /// Close the file handle.
    close_file: extern "efiapi" fn(
        handle: ShellFileHandle
    ) -> Status,
    /// Create a file or directory by name.
    create_file: extern "efiapi" fn(
        name: *const u16,
        attrs: u64,
        handle: *mut ShellFileHandle,
    ) -> Status,
    /// Read data from a file.
    read_file: extern "efiapi" fn(
        handle: ShellFileHandle,
        size: *mut usize,
        buffer: *mut c_void
    ) -> Status,
    /// Write data to a file.
    write_file: extern "efiapi" fn(
        handle: ShellFileHandle,
        size: *mut usize,
        buffer: *mut c_void
    ) -> Status,
    /// Delete the file specified by the file handle.
    delete_file: extern "efiapi" fn(
        handle: ShellFileHandle
    ) -> Status,
    /// Delete the file specified by the file name.
    delete_file_by_name: extern "efiapi" fn(
        name: *const u16
    ) -> Status,
    /// Get a file's current position.
    get_file_position: extern "efiapi" fn(
        handle: ShellFileHandle,
        position: *mut u64
    ) -> Status,
    /// Set a file's current position.
    set_file_position: extern "efiapi" fn(
        handle: ShellFileHandle,
        position: u64
    ) -> Status,
    /// Flush data back to a device.
    flush_file: extern "efiapi" fn(
        handle: ShellFileHandle
    ) -> Status,
    /// Find files that match a specified pattern.
    find_files: extern "efiapi" fn(
        pattern: *const u16,
        list: *mut *mut ShellFileInfo
    ) -> Status,
    /// Find all files in a specified directory.
    find_files_in_dir: extern "efiapi" fn(
        dir:  ShellFileHandle,
        list: *mut *mut ShellFileInfo
    ) -> Status,
    /// Get the size of a file.
    get_file_size: extern "efiapi" fn(
        handle: ShellFileHandle,
        size: *mut u64
    ) -> Status,

    /// Open the root directory of a device.
    open_root: extern "efiapi" fn(
        device: *mut DevicePath,
        handle: *mut ShellFileHandle
    ) -> Status,
    /// Open the root directory of a device on a handle.
    open_root_by_handle: extern "efiapi" fn(
        device: Handle,
        handle: *mut ShellFileHandle
    ) -> Status,

    /// Event signaled by the UEFI Shell when the user presses Ctrl-C to
    /// indicate that the current UEFI Shell command execution should be
    /// interrupted.
    execution_break: Event,

    /// The EFI_SHELL_MAJOR_VERSION value. Defines what functions are available
    /// in the protocol.
    major_version: u32,
    /// The EFI_SHELL_MINOR_VERSION value. Defines what functions are available
    /// in the protocol.
    minor_version: u32,

    /// Register a GUID and a localized human-readable name for it.
    register_guid_name: extern "efiapi" fn(
        guid: *const Guid,
        name: *const u16
    ) -> Status,
    /// Get the human-readable name for a GUID from the value.
    get_guid_name: extern "efiapi" fn(
        guid: *const Guid,
        name: *mut *const u16
    ) -> Status,
    /// Get the GUID value from a human-readable name.
    get_guid_from_name: extern "efiapi" fn(
        name: *const u16,
        guid: *mut Guid
    ) -> Status,
    /// Get the environment variable and Attributes, or list of environment
    /// variables.
    get_env_ex: extern "efiapi" fn(
        name: *const u16,
        attrs: *mut u32
    ) -> *const u16,
}

/// Shell application's arguments.
#[repr(C)]
#[unsafe_guid("752F3136-4E16-4FDC-A22A-E5F46812F4CA")]
#[derive(Protocol)]
pub struct ShellParametersProtocol {
    argv: *const *const u16,
    argc: usize,
    stdin: ShellFileHandle,
    stdout: ShellFileHandle,
    stderr: ShellFileHandle,
}

/// Advertise an external shell command.
#[repr(C)]
#[unsafe_guid("3C7200E9-005F-4EA4-87DE-A3DFAC8A27C3")]
#[derive(Protocol)]
pub struct ShellDynamicCommandProtocol {
    command_name: *const u16,
    handler: *const c_void,
    get_help: *const c_void,
}

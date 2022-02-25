// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

//! UEFI Shell status codes
//!
//! ## References
//!
//! - [UEFI Shell Specification, Revision 2.2][UEFI-Shell]: Appendix C - UEFI
//!   Shell Status Codes
//!
//! [UEFI-Shell]: https://uefi.org/sites/default/files/resources/UEFI_Shell_2_2.pdf

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd)]
#[must_use]
#[repr(transparent)]
pub struct ShellStatus(usize);

impl ShellStatus {
    /// The operation completed successfully.
    pub const SUCCESS: Self = Self(0);
    /// The image failed to load.
    pub const LOAD_ERROR: Self = Self(1);
    /// There was an error in the command-line options.
    pub const INVALID_PARAMETER: Self = Self(2);
    /// The operation is not supported.
    pub const UNSUPPORTED: Self = Self(3);
    /// The buffer was not the proper size for the request.
    pub const BAD_BUFFER_SIZE: Self = Self(4);
    /// The buffer is not large enough to hold the requested data. The required
    /// buffer size is returned in the appropriate parameter when this occurs.
    pub const BUFFER_TOO_SMALL: Self = Self(5);
    /// There is no data pending upon return.
    pub const NOT_READY: Self = Self(6);
    /// The physical device reported an error while attempting the operation.
    pub const DEVICE_ERROR: Self = Self(7);
    /// The device cannot be written to.
    pub const WRITE_PROTECTED: Self = Self(8);
    /// A resource has run out.
    pub const OUT_OF_RESOURCES: Self = Self(9);
    /// An inconstancy was detected on the file system causing the operation to
    /// fail.
    pub const VOLUME_CORRUPTED: Self = Self(10);
    /// There is no more space on the file system.
    pub const VOLUME_FULL: Self = Self(11);
    /// The device does not contain any medium to perform the operation.
    pub const NO_MEDIA: Self = Self(12);
    /// The medium in the device has changed since the last access.
    pub const MEDIA_CHANGED: Self = Self(13);
    /// The item was not found.
    pub const NOT_FOUND: Self = Self(14);
    /// Access was denied.
    pub const ACCESS_DENIED: Self = Self(15);

    /// The timeout time expired.
    pub const TIMEOUT: Self = Self(18);
    /// The specified operation cound not be started.
    pub const NOT_STARTED: Self = Self(19);
    /// The specified operation has already started.
    pub const ALREADY_STARTED: Self = Self(20);
    /// The operation was aborted by the user.
    pub const ABORTED: Self = Self(21);

    /// The function encountered an internal version that was incompatible with
    /// a version requested by the caller.
    pub const INCOMPATIBLE_VERSION: Self = Self(25);
    /// The function was not performed due to a security violation.
    pub const SECURITY_VIOLATION: Self = Self(26);
    /// The function was performed and resulted in an unequal comparison.
    pub const NOT_EQUAL: Self = Self(27);
}

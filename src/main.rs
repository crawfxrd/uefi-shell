// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(negative_impls)]
#![allow(unreachable_code)]
#![allow(dead_code)]

extern crate alloc;

mod data_types;
mod proto;
mod status;

use uefi::prelude::*;
use uefi::CStr16;

#[entry]
fn main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap_success();

    let _ = st.boot_services().set_watchdog_timer(0, 0x1_0000, None).unwrap_success();
    let _ = st.stdout().clear().unwrap_success();
    let _ = st.stdout().enable_cursor(true).unwrap_success();

    let mut buf = [0; 32];
    let test_string = CStr16::from_str_with_buf("uefi-shell", &mut buf).unwrap();
    let _ = st.stdout().output_string(test_string);

    let mut ev = unsafe { [st.stdin().wait_for_key_event().unsafe_clone()] };
    st.boot_services().wait_for_event(&mut ev).unwrap_success();

    Status::SUCCESS
}

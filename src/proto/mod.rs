// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2022 System76

pub mod dynamic;
pub use dynamic::ShellDynamicCommandProtocol;

pub mod shell;
pub use shell::ShellProtocol;

pub mod parameters;
pub use parameters::ShellParametersProtocol;

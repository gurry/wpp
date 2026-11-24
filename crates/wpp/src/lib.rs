// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! WPP software tracing runtime for Rust drivers.
//!
//! Provides the runtime types and ETW bindings needed by the code generated
//! by `wpp_control_guids!` and `trace!` macros.

#![no_std]

#[cfg(all(feature = "kernel_mode", feature = "user_mode"))]
compile_error!("wpp: `kernel_mode` and `user_mode` are mutually exclusive");

#[cfg(not(any(feature = "kernel_mode", feature = "user_mode")))]
compile_error!("wpp: enable exactly one of `kernel_mode` or `user_mode`");

extern crate alloc;

pub mod etw;
pub mod field;
pub mod ifr;
pub mod provider;

pub use field::{IntoWppField, WppField};
pub use ifr::IFRState;
pub use provider::ProviderState;
pub use wpp_macros::{__wpp_trace_impl, WppField, wpp_control_guids};

/// GUID layout matching the Windows GUID structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

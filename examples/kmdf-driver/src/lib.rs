//! A barebones sample KMDF driver
//! Demonstrates WPP tracing

#![no_std]
#![feature(codeview_annotation)]
#![feature(core_os)]

use wdf::{
    driver_entry, wpp_control_guids, DeviceInit, Driver, DriverConfig, DriverObject,
    status_codes, IRQL, NtResult, NtStatus, UnicodeString,
};

extern crate alloc;

wpp_control_guids!(
    DriverTracer "f6912289-953b-48fa-b213-44bf82ae9623" {
        GENERAL,
        DEVICE,
    }
);

// Modules declared after wpp_control_guids! so trace macros are in scope
mod device;

#[driver_entry(trace_providers(DriverTracer))]
fn driver_entry(driver_object: &mut DriverObject, registry_path: &UnicodeString) -> NtResult<()> {
    let config = DriverConfig::new(evt_device_add);
    let _driver = match Driver::create(driver_object, registry_path, config) {
        Ok(driver) => driver,
        Err(status) => {
            trace!(
                INFO, GENERAL,
                "Driver entry completed with status = {}",
                status
            );

            return Err(status)
        },
    };

    Ok(())
}


fn evt_device_add(device_init: &mut DeviceInit) -> NtResult<()> {
    trace!(VERBOSE, "Enter evt_device_add");
    device::device_create(device_init)
}

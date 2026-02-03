use std::ffi::CString;
use std::ptr::null_mut;
use libc::{c_char, c_int};
use crate::c_helper_fun::c_str_to_string;
use crate::cups::enums::http_encryption::HttpEncryption;
use crate::cups::ipp::IppRaw;
use crate::cups::enums::ipp_status::IppStatus;
use crate::cups::enums::protocol_families::PF;

#[repr(C)]
pub struct HttpT { _private: [u8; 0] }

#[repr(C)]
pub struct HttpAddrlistT { _private: [u8; 0] }

unsafe extern "C" {
    fn cupsServer() -> *const c_char;
    fn ippPort() -> c_int;
    fn httpConnect2(
        host: *const c_char,
        port: c_int,
        addrlist: *mut HttpAddrlistT,
        family: c_int,
        encryption: HttpEncryption,
        blocking: c_int,
        msec: c_int,
        cancel: *mut c_int,
    ) -> *mut HttpT;

    fn cupsDoRequest(
        http: *mut HttpT,
        request: *mut IppRaw,
        resource: *const c_char,
    ) -> *mut IppRaw;

    fn cupsLastError() -> IppStatus;
    fn cupsLastErrorString() -> *const c_char;
    fn httpClose(http: *mut HttpT);
}

pub fn cups_server() -> Option<String> {
    let server_ptr = unsafe { cupsServer() };
    c_str_to_string(server_ptr)
}

pub fn ipp_port() -> i32 {
    unsafe { ippPort() as i32 }
}

pub fn http_connect2(
    host: &str,
    port: i32,
    addrlist: Option<*mut HttpAddrlistT>,
    family: PF,
    encryption: HttpEncryption,
    blocking: bool,
    msec: i32,
    cancel: Option<&mut i32>,
) -> Option<*mut HttpT> {
    let c_host = CString::new(host).ok()?;
    let addr = addrlist.unwrap_or(null_mut());
    let blocking_c = if blocking { 1 } else { 0 };
    let cancel_ptr = match cancel {
        Some(r) => r as *mut i32,
        None => null_mut(),
    };

    let res = unsafe {
        httpConnect2(
            c_host.as_ptr(),
            port as c_int,
            addr,
            family as c_int,
            encryption,
            blocking_c as c_int,
            msec as c_int,
            cancel_ptr as *mut c_int,
        )
    };

    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn cups_do_request(
    http: *mut HttpT,
    request: *mut IppRaw,
    resource: &str,
) -> Option<*mut IppRaw> {
    let c_resource = CString::new(resource).ok()?;

    let res = unsafe {
        cupsDoRequest(
            http,
            request,
            c_resource.as_ptr(),
        )
    };

    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn cups_last_error() -> IppStatus {
    unsafe { cupsLastError() }
}

pub fn cups_last_error_string() -> Option<String> {
    let err_ptr = unsafe { cupsLastErrorString() };
    c_str_to_string(err_ptr)
}

pub fn http_close(http: *mut HttpT) {
    unsafe {
        httpClose(http);
    }
}
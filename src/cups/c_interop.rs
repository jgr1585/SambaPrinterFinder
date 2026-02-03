use std::ffi::CString;
use std::ptr::{null, null_mut};
use libc::{c_char, c_int};
use crate::c_helper_fun::c_str_to_string;
use crate::cups::http_encryption::HttpEncryption;
use crate::cups::ipp_operations::IppOp;
use crate::cups::ipp_status::IppStatus;
use crate::cups::ipp_tag::IPPTag;
use crate::cups::protocol_families::PF;

#[repr(C)]
pub struct HttpT { _private: [u8; 0] }

#[repr(C)]
pub struct HttpAddrlistT { _private: [u8; 0] }

#[repr(C)]
pub struct IppAttributeT { _private: [u8; 0] }

#[repr(C)]
pub struct IppT { _private: [u8; 0] }

unsafe extern "C" {
    fn cupsServer() -> *const c_char;
    fn ippPort() -> c_int;
    fn ippNewRequest(op: c_int) -> *mut IppT;
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

    fn ippAddString(
        ipp: *mut IppT,
        group: IPPTag,
        value_tag: IPPTag,
        name: *const c_char,
        language: *const c_char,
        value: *const c_char,
    ) -> *mut IppAttributeT;

    fn cupsDoRequest(
        http: *mut HttpT,
        request: *mut IppT,
        resource: *const c_char,
    ) -> *mut IppT;

    fn cupsLastError() -> IppStatus;
    fn cupsLastErrorString() -> *const c_char;
    fn ippDelete(ipp: *mut IppT);
    fn httpClose(http: *mut HttpT);
    fn ippFirstAttribute(ipp: *mut IppT) -> *mut IppAttributeT;
    fn ippNextAttribute(ipp: *mut IppT) -> *mut IppAttributeT;
    fn ippGetName(attr: *mut IppAttributeT) -> *const c_char;
    fn ippGetGroupTag(attr: *mut IppAttributeT) -> IPPTag;
    fn ippGetString(
        attr: *mut IppAttributeT,
        element: c_int,
        c_char: *mut c_char
    ) -> *const c_char;
}

pub fn cups_server() -> Option<String> {
    let server_ptr = unsafe { cupsServer() };
    c_str_to_string(server_ptr)
}

pub fn ipp_port() -> i32 {
    unsafe { ippPort() as i32 }
}

pub fn ipp_new_request(op: IppOp) -> Option<*mut IppT> {
    let res = unsafe { ippNewRequest(op as c_int) };
    if res.is_null() {
        None
    } else {
        Some(res)
    }
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

pub fn ipp_add_string(
    ipp: Option<*mut IppT>,
    group: IPPTag,
    value_tag: IPPTag,
    name: Option<&str>,
    language: Option<&str>,
    value: &str,
) -> Option<*mut IppAttributeT> {
    let c_name = name.and_then(|s| CString::new(s).ok());
    let c_language = language.and_then(|s| CString::new(s).ok());
    let c_value = CString::new(value).ok()?;

    let res = unsafe {
        ippAddString(
            ipp.unwrap_or(null_mut()),
            group,
            value_tag,
            c_name.as_ref().map_or(null(), |s| s.as_ptr()),
            c_language.as_ref().map_or(null(), |s| s.as_ptr()),
            c_value.as_ptr(),
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
    request: Option<*mut IppT>,
    resource: &str,
) -> Option<*mut IppT> {
    let c_resource = CString::new(resource).ok()?;

    let res = unsafe {
        cupsDoRequest(
            http,
            request.unwrap_or(null_mut()),
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

pub fn ipp_delete(ipp: Option<*mut IppT>) {
    if let Some(ipp_ptr) = ipp {
        unsafe {
            ippDelete(ipp_ptr);
        }
    }
}

pub fn http_close(http: *mut HttpT) {
    unsafe {
        httpClose(http);
    }
}

pub fn ipp_first_attribute(ipp: Option<*mut IppT>) -> Option<*mut IppAttributeT> {
    let res = unsafe { ippFirstAttribute(ipp.unwrap_or(null_mut())) };
    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn ipp_next_attribute(ipp: Option<*mut IppT>) -> Option<*mut IppAttributeT> {
    let res = unsafe { ippNextAttribute(ipp.unwrap_or(null_mut())) };
    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn ipp_get_group_tag(attr: Option<*mut IppAttributeT>) -> IPPTag {
    unsafe { ippGetGroupTag(attr.unwrap_or(null_mut())) }
}

pub fn ipp_get_string(
    attr: Option<*mut IppAttributeT>,
    element: i32,
) -> Option<String> {
    let res_ptr = unsafe {
        ippGetString(
            attr.unwrap_or(null_mut()),
            element as c_int, null_mut(),
        )
    };

    c_str_to_string(res_ptr)
}

pub fn ipp_get_name(attr: Option<*mut IppAttributeT>) -> Option<String> {
    let name_ptr = unsafe { ippGetName(attr.unwrap_or(null_mut())) };
    c_str_to_string(name_ptr)
}
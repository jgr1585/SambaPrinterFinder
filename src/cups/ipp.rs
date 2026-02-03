use std::ffi::CString;
use std::ptr::null;
use libc::{c_char, c_int};
use crate::cups::ipp_attribute::IppAttribute;
use crate::cups::enums::ipp_operations::IppOp;
use crate::cups::enums::ipp_tag::IPPTag;

// IPP request/response data (opaque C struct)
#[repr(C)]
pub(super) struct IppRaw { _private: [u8; 0] }

pub struct Ipp {
    raw: *mut IppRaw,
}

unsafe extern "C" {
    fn ippNewRequest(op: c_int) -> *mut IppRaw;
    fn ippAddString(
        ipp: *mut IppRaw,
        group: IPPTag,
        value_tag: IPPTag,
        name: *const c_char,
        language: *const c_char,
        value: *const c_char,
    ) -> *mut IppAttribute;

    fn ippFirstAttribute(ipp: *mut IppRaw) -> *mut IppAttribute;
    fn ippNextAttribute(ipp: *mut IppRaw) -> *mut IppAttribute;

    fn ippDelete(ipp: *mut IppRaw);
}

impl Ipp {
    pub fn new(op: IppOp) -> Self {
        let res = unsafe { ippNewRequest(op as c_int) };
        if res.is_null() {
            panic!("Failed to create new IPP request");
        } else {
            Self { raw: res }
        }
    }

    pub fn from_raw(raw: *mut IppRaw) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub fn into_raw(self) -> *mut IppRaw {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    pub fn add_string(
        &self,
        group: IPPTag,
        value_tag: IPPTag,
        name: Option<&str>,
        language: Option<&str>,
        value: &str,
    ) -> Option<*mut IppAttribute> {
        let c_name = name.and_then(|s| CString::new(s).ok());
        let c_language = language.and_then(|s| CString::new(s).ok());
        let c_value = CString::new(value).ok()?;

        let res = unsafe {
            ippAddString(
                self.raw,
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

    pub fn get_first_attribute(&mut self) -> Option<*mut IppAttribute> {
        let res = unsafe { ippFirstAttribute(self.raw) };
        if res.is_null() {
            None
        } else {
            Some(res)
        }
    }


    pub fn get_next_attribute(&mut self) -> Option<*mut IppAttribute> {
        let res = unsafe { ippNextAttribute(self.raw) };
        if res.is_null() {
            None
        } else {
            Some(res)
        }
    }
}

impl Drop for Ipp {
    fn drop(&mut self) {
        unsafe {
            if !self.raw.is_null() {
                ippDelete(self.raw);
            }
        }
    }
}
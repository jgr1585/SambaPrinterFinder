use std::ptr::null_mut;
use libc::{c_char, c_int};
use crate::c_helper_fun::c_str_to_string;
use crate::cups::enums::ipp_tag::IPPTag;

// IPP Attribute data
#[repr(C)]
pub struct IppAttribute { _private: [u8; 0] }

unsafe extern "C" {
    fn ippGetName(attr: *mut IppAttribute) -> *const c_char;
    fn ippGetGroupTag(attr: *mut IppAttribute) -> IPPTag;
    fn ippGetString(
        attr: *mut IppAttribute,
        element: c_int,
        c_char: *mut c_char
    ) -> *const c_char;
}

impl IppAttribute {
    pub fn get_group_tag(&mut self) -> IPPTag {
        unsafe { ippGetGroupTag(self) }
    }

    pub fn get_string(&mut self) -> Option<String> {
        let res_ptr = unsafe {
            ippGetString(self, 0, null_mut())
        };

        c_str_to_string(res_ptr)
    }

    pub fn get_name(&mut self) -> Option<String> {
        let name_ptr = unsafe { ippGetName(self) };
        c_str_to_string(name_ptr)
    }
}
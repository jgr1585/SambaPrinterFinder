// Helper functions for C interoperability

use std::ffi::{c_char, CStr};

// Helper function to convert a Rust string to a C string
pub fn c_str_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    unsafe {
        CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_owned())
    }
}
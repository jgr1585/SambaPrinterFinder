use std::cmp::Ord;
use std::default::Default;
use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use once_cell::sync::Lazy;
use std::prelude::v1::{Err, Ok};
use std::{ptr, slice};
use std::string::{String, ToString};
use std::sync::Mutex;
use std::vec::Vec;
use libc::{c_int};

#[repr(C)]
pub struct SMBCCTX {
    _private: [i32; 0],
}

#[repr(C)]
pub struct SMBCFILE {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmbcType {
    SmbcWorkgroup = 1,
    SmbcServer = 2,
    SmbcFileShare = 3,
    SmbcPrinterShare = 4,
    SmbcCommsShare = 5,
    SmbcIpcShare = 6,
    SmbcDir = 7,
    SmbcFile = 8,
    SmbcLink = 9,
}

#[repr(C)]
pub struct SmbcDirent {
    pub smbc_type: SmbcType,
    pub dirlen: u32,
    pub commentlen: u32,
    pub comment: *mut i8,
    pub namelen: u32,
    pub name: [i8; 1],
}


extern "C" {
    fn smbc_new_context() -> *mut SMBCCTX;
    fn smbc_init_context(ctx: *mut SMBCCTX) -> *mut SMBCCTX;
    fn smbc_free_context(ctx: *mut SMBCCTX, shutdown_ctx: c_int);
    fn smbc_set_context(ctx: *mut SMBCCTX) -> *mut SMBCCTX;
    fn smbc_setFunctionAuthData(
        ctx: *mut SMBCCTX,
        func: unsafe extern "C" fn(*const i8, *const i8, *mut i8, i32, *mut i8, i32, *mut i8, i32),
    );
    fn smbc_opendir(path: *const i8) -> *mut SMBCFILE;
    fn smbc_readdir(dir: *mut SMBCFILE) -> *mut SmbcDirent;
    fn smbc_closedir(dir: *mut SMBCFILE) -> c_int;
}

#[derive(Clone)]
pub struct SambaCredentials {
    pub workgroup: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct SambaConnection {
    ctx: *mut SMBCCTX,
    pub credentials: SambaCredentials
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SambaEntryType {
    File,
    Directory,
    Printer,
    Unknown,
}

impl Default for SambaEntryType {
    fn default() -> Self {
        SambaEntryType::Unknown
    }
}

pub struct SambaDirectoryEntry {
    pub name: String,
    pub entry_type: SambaEntryType,
}

static CREDENTIALS: Lazy<Mutex<SambaCredentials>> = Lazy::new(|| {
    Mutex::new(SambaCredentials {
        workgroup: String::new(),
        username: String::new(),
        password: String::new(),
    })
});


impl SambaConnection {
    pub fn connect(credentials: SambaCredentials) -> Result<Self> {
        unsafe {
            let ctx = smbc_new_context();
            if ctx.is_null() {
                return Err(Error::new(ErrorKind::Other, "Failed to create SMB context"));
            }

            // sichere Initialisierung der globalen Credentials
            {
                let mut g = CREDENTIALS.lock().unwrap();
                *g = credentials.clone();
            }

            smbc_setFunctionAuthData(ctx, auth_fn);

            let ctx = smbc_init_context(ctx);
            if ctx.is_null() {
                return Err(Error::new(ErrorKind::Other, "Failed to init SMB context"));
            }

            Ok(SambaConnection { ctx, credentials } )
        }
    }

    pub fn disconnect(&self) {
        unsafe {
            smbc_free_context(self.ctx, 1);
        }
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<SambaDirectoryEntry>> {
        let mut entries: Vec<SambaDirectoryEntry> = Vec::new();

        unsafe {
            // set the context for this thread
            smbc_set_context(self.ctx);

            // convert Rust &str to a null-terminated C string
            let c_path = CString::new(path).map_err(|_| {
                Error::new(ErrorKind::InvalidInput, "path contains interior NUL byte")
            })?;

            let dir = smbc_opendir(c_path.as_ptr());
            if dir.is_null() {
                return Err(Error::new(ErrorKind::Other, "Failed to open directory"));
            }

            loop {
                let entry = smbc_readdir(dir);

                if entry.is_null() {
                    break;
                }

                let name_ptr = (*entry).name.as_ptr();
                let name_len = (*entry).namelen as usize;
                let name_slice = slice::from_raw_parts(name_ptr as *const u8, name_len);
                let name = String::from_utf8_lossy(name_slice).to_string();

                let entry_type = match (*entry).smbc_type {
                    SmbcType::SmbcDir => SambaEntryType::Directory,
                    SmbcType::SmbcFileShare => SambaEntryType::Directory,
                    SmbcType::SmbcFile => SambaEntryType::File,
                    SmbcType::SmbcPrinterShare => SambaEntryType::Printer,
                    _ => SambaEntryType::Unknown,
                };

                entries.push(SambaDirectoryEntry {
                    name,
                    entry_type,
                });
            }

            smbc_closedir(dir);

            // Removes the context for this thread
            smbc_set_context(ptr::null_mut());
        }

        Ok(entries)
    }
}

pub unsafe extern "C" fn auth_fn(
    _server: *const i8,
    _share: *const i8,
    workgroup: *mut i8, wglen: i32,
    username: *mut i8, unlen: i32,
    password: *mut i8, pwlen: i32,
) {
    unsafe fn write_c_string(dst: *mut i8, len: i32, src: &str) {
        if dst.is_null() || len <= 0 {
            return;
        }

        let bytes = src.as_bytes();
        let max = (len as usize).saturating_sub(1);
        let n = bytes.len().min(max);

        ptr::copy_nonoverlapping(bytes.as_ptr(), dst as *mut u8, n);
        *dst.add(n) = 0; // null-terminate
    }

    let credentials = CREDENTIALS.lock().unwrap();

    write_c_string(workgroup, wglen, credentials.workgroup.as_str());
    write_c_string(username,  unlen, credentials.username.as_str());
    write_c_string(password,  pwlen, credentials.password.as_str());
}


use objc::runtime::{Object, Class};
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;
use std::path::PathBuf;
use std::ffi::CStr;

pub fn get_resource_path() -> Option<PathBuf> {
    unsafe {
        // Get the main bundle
        let ns_bundle: *mut Object = msg_send![Class::get("NSBundle").unwrap(), mainBundle];
        
        // Get the resource path as an NSString
        let ns_string: *mut Object = msg_send![ns_bundle, resourcePath];
        
        if ns_string.is_null() {
            return None;
        }
        
        // Convert NSString to a Rust string
        let c_str: *const libc::c_char = msg_send![ns_string, UTF8String];
        let path = CStr::from_ptr(c_str).to_string_lossy().into_owned();
        
        Some(PathBuf::from(path))
    }
}
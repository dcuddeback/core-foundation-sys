// Exports from <CoreFoundation/CFCharacterSet.h>

use libc::c_void;

#[repr(C)]
struct __CFCharacterSet {
    __private: c_void,
}

pub type CFCharacterSetRef = *const __CFCharacterSet;
pub type CFMutableCharacterSetRef = *mut __CFCharacterSet;

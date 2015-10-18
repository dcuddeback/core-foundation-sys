// Exports of <CoreFoundation/CFLocale.h>

use libc::c_void;

#[repr(C)]
struct __CFLocale {
    __private: c_void,
}

pub type CFLocaleRef = *const __CFLocale;

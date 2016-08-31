// Exports of <CoreFoundation/CFLocale.h>

use libc::c_void;

#[doc(hidden)]
#[repr(C)]
pub struct __CFLocale {
    __private: c_void,
}

pub type CFLocaleRef = *const __CFLocale;

// Exports of <CoreFoundation/CFNumber.h>

use libc::c_void;

use base::*;

#[doc(hidden)]
#[repr(C)]
pub struct __CFBoolean {
    __private: c_void,
}

pub type CFBooleanRef = *const __CFBoolean;

extern "C" {
    pub fn CFBooleanGetTypeID() -> CFTypeID;
    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
}

pub type CFNumberType = CFIndex;
pub const kCFNumberSInt8Type: CFNumberType = 1;
pub const kCFNumberSInt16Type: CFNumberType = 2;
pub const kCFNumberSInt32Type: CFNumberType = 3;
pub const kCFNumberSInt64Type: CFNumberType = 4;
pub const kCFNumberFloat32Type: CFNumberType = 5;
pub const kCFNumberFloat64Type: CFNumberType = 6;
pub const kCFNumberCharType: CFNumberType = 7;
pub const kCFNumberShortType: CFNumberType = 8;
pub const kCFNumberIntType: CFNumberType = 9;
pub const kCFNumberLongType: CFNumberType = 10;
pub const kCFNumberLongLongType: CFNumberType = 11;
pub const kCFNumberFloatType: CFNumberType = 12;
pub const kCFNumberDoubleType: CFNumberType = 13;
pub const kCFNumberCFIndexType: CFNumberType = 14;
pub const kCFNumberNSIntegerType: CFNumberType = 15;
pub const kCFNumberCGFloatType: CFNumberType = 16;
pub const kCFNumberMaxType: CFNumberType = 16;

#[doc(hidden)]
#[repr(C)]
pub struct __CFNumber {
    __private: c_void,
}

pub type CFNumberRef = *const __CFNumber;

extern "C" {
    pub fn CFNumberGetTypeID() -> CFTypeID;
    pub fn CFNumberCreate(alloc: CFAllocatorRef, theType: CFNumberType, valuePtr: *const c_void) -> CFNumberRef;
    pub fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
    pub fn CFNumberGetByteSize(number: CFNumberRef) -> CFIndex;
    pub fn CFNumberIsFloatType(number: CFNumberRef) -> Boolean;
    pub fn CFNumberGetValue(number: CFNumberRef, theType: CFNumberType, valuePtr: *mut c_void) -> Boolean;
    pub fn CFNumberCompare(number: CFNumberRef, otherNumber: CFNumberRef, context: *mut c_void) -> CFComparisonResult;
}

// Exports from <CoreFoundation/CFBase.h>

use libc::{c_void,c_uchar,c_long,c_ulong};

pub type Boolean = c_uchar;

pub type UInt8 = u8;
pub type SInt8 = i8;
pub type UInt16 = u16;
pub type SInt16 = i16;
pub type UInt32 = u32;
pub type SInt32 = i32;

pub type UnicodeScalarValue = UInt32;
pub type UTF32Char          = UInt32;
pub type UniChar            = UInt16;
pub type UTF16Char          = UInt16;
pub type UTF8Char           = UInt8;
pub type UniCharPtr         = *mut UniChar;
pub type UniCharCount       = c_ulong;
pub type UniCharCountPtr    = *mut UniCharCount;
pub type Str255             = [c_uchar; 256];
pub type Str63              = [c_uchar; 64];
pub type Str32              = [c_uchar; 33];
pub type Str31              = [c_uchar; 32];
pub type Str27              = [c_uchar; 28];
pub type Str15              = [c_uchar; 16];

pub type CFTypeID = c_ulong;
pub type CFOptionFlags = c_ulong;
pub type CFHashCode = c_ulong;
pub type CFIndex = c_long;

pub type CFTypeRef = *const c_void;

#[doc(hidden)]
#[repr(C)]
pub struct __CFString {
    __private: c_void,
}

pub type CFStringRef = *const __CFString;
pub type CFMutableStringRef = *mut __CFString;

pub type CFComparisonResult = CFIndex;
pub const kCFCompareLessThan:    CFComparisonResult = -1;
pub const kCFCompareEqualTo:     CFComparisonResult = 0;
pub const kCFCompareGreaterThan: CFComparisonResult = 1;

pub type CFComparatorFunction = extern "C" fn(val1: *const c_void, val2: *const c_void) -> CFComparisonResult;

pub const kCFNotFound: CFIndex = -1;

#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex
}

#[doc(hidden)]
#[repr(C)]
pub struct __CFAllocator {
    __private: c_void,
}

pub type CFAllocatorRef = *const __CFAllocator;

pub type CFAllocatorRetainCallBack = extern "C" fn(info: *const c_void) -> *const c_void;
pub type CFAllocatorReleaseCallBack = extern "C" fn(info: *const c_void);
pub type CFAllocatorCopyDescriptionCallBack = extern "C" fn(info: *const c_void) -> CFTypeRef;
pub type CFAllocatorAllocateCallBack = extern "C" fn(allocSize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void;
pub type CFAllocatorReallocateCallBack = extern "C" fn(ptr: *mut c_void, newSize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void;
pub type CFAllocatorDeallocateCallBack = extern "C" fn(ptr: *mut c_void, info: *mut c_void);
pub type CFAllocatorPreferredSizeCallBack = extern "C" fn(size: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> CFIndex;

#[repr(C)]
pub struct CFAllocatorContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
    pub allocate: CFAllocatorAllocateCallBack,
    pub reallocate: CFAllocatorReallocateCallBack,
    pub deallocate: CFAllocatorDeallocateCallBack,
    pub preferredSize: CFAllocatorPreferredSizeCallBack
}

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
    pub static kCFAllocatorMalloc: CFAllocatorRef;
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
    pub static kCFAllocatorNull: CFAllocatorRef;
    pub static kCFAllocatorUseContext: CFAllocatorRef;

    pub fn CFAllocatorGetTypeID() -> CFTypeID;

    pub fn CFAllocatorSetDefault(allocator: CFAllocatorRef);
    pub fn CFAllocatorGetDefault() -> CFAllocatorRef;

    pub fn CFAllocatorCreate(allocator: CFAllocatorRef, context: *mut CFAllocatorContext) -> CFAllocatorRef;
    pub fn CFAllocatorAllocate(allocator: CFAllocatorRef, size: CFIndex, hint: CFOptionFlags) -> *mut c_void;
    pub fn CFAllocatorReallocate(allocator: CFAllocatorRef, ptr: *mut c_void, newSize: CFIndex, hint: CFOptionFlags) -> *mut c_void;
    pub fn CFAllocatorDeallocate(allocator: CFAllocatorRef, ptr: *mut c_void);
    pub fn CFAllocatorGetPreferredSizeForSize(allocator: CFAllocatorRef, size: CFIndex, hint: CFOptionFlags) -> CFIndex;
    pub fn CFAllocatorGetContext(allocator: CFAllocatorRef, context: *mut CFAllocatorContext);

    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFCopyTypeIDDescription(cf: CFTypeID) -> CFStringRef;

    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFAutorelease(arg: CFTypeRef) -> CFTypeRef;
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;

    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;

    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;

    pub fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;
    pub fn CFMakeCollectable(cf: CFTypeRef) -> CFTypeRef;
}

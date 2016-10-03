// Exports from <CoreFoundation/CFData.h>

use libc::c_void;

use base::*;

#[doc(hidden)]
#[repr(C)]
pub struct __CFData {
    __private: c_void,
}

pub type CFDataRef = *const __CFData;
pub type CFMutableDataRef = *mut __CFData;

pub type CFDataSearchFlags = CFOptionFlags;
pub const kCFDataSearchBackwards: CFDataSearchFlags = 1 << 0;
pub const kCFDataSearchAnchored: CFDataSearchFlags = 1 << 1;

extern "C" {
    pub fn CFDataGetTypeID() -> CFTypeID;

    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const UInt8, length: CFIndex) -> CFDataRef;
    pub fn CFDataCreateWithBytesNoCopy(allocator: CFAllocatorRef, bytes: *const UInt8, length: CFIndex, bytesDeallocator: CFAllocatorRef) -> CFDataRef;
    pub fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
    pub fn CFDataCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex) -> CFMutableDataRef;
    pub fn CFDataCreateMutableCopy(allocator: CFAllocatorRef, capacity: CFIndex, theData: CFDataRef) -> CFMutableDataRef;

    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const UInt8;
    pub fn CFDataGetMutableBytePtr(theData: CFMutableDataRef) -> *mut UInt8;
    pub fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *mut UInt8);

    pub fn CFDataSetLength(theData: CFMutableDataRef, length: CFIndex);
    pub fn CFDataIncreaseLength(theData: CFMutableDataRef, extraLength: CFIndex);
    pub fn CFDataAppendBytes(theData: CFMutableDataRef, bytes: *const UInt8, length: CFIndex);
    pub fn CFDataReplaceBytes(theData: CFMutableDataRef, range: CFRange, newBytes: *const UInt8, newLength: CFIndex);
    pub fn CFDataDeleteBytes(theData: CFMutableDataRef, range: CFRange);

    pub fn CFDataFind(theData: CFDataRef, dataToFind: CFDataRef, searchRange: CFRange, compareOptions: CFDataSearchFlags) -> CFRange;
}

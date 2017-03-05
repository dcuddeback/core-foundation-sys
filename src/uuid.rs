use libc::c_void;
use ::{CFAllocatorRef, CFStringRef, CFTypeID, UInt8};

#[doc(hidden)]
#[repr(C)]
pub struct __CFUUID {
    __private: c_void
}

pub type CFUUIDRef = *const __CFUUID;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CFUUIDBytes {
    byte0: UInt8,
    byte1: UInt8,
    byte2: UInt8,
    byte3: UInt8,
    byte4: UInt8,
    byte5: UInt8,
    byte6: UInt8,
    byte7: UInt8,
    byte8: UInt8,
    byte9: UInt8,
    byte10: UInt8,
    byte11: UInt8,
    byte12: UInt8,
    byte13: UInt8,
    byte14: UInt8,
    byte15: UInt8
}

extern "C" {
    pub fn CFUUIDGetTypeID() -> CFTypeID;

    pub fn CFUUIDCreate(alloc: CFAllocatorRef) -> CFUUIDRef;
    pub fn CFUUIDCreateWithBytes(alloc: CFAllocatorRef, byte0: UInt8, byte1: UInt8, byte2: UInt8, byte3: UInt8,
                                 byte4: UInt8, byte5: UInt8, byte6: UInt8, byte7: UInt8, byte8: UInt8, byte9: UInt8,
                                 byte10: UInt8, byte11: UInt8, byte12: UInt8, byte13: UInt8, byte14: UInt8, byte15: UInt8)
                                 -> CFUUIDRef;
    pub fn CFUUIDCreateFromString(alloc: CFAllocatorRef, uuidStr: CFStringRef) -> CFUUIDRef;
    pub fn CFUUIDCreateString(alloc: CFAllocatorRef, uuid: CFUUIDRef) -> CFStringRef;
    pub fn CFUUIDGetConstantUUIDWithBytes(alloc: CFAllocatorRef, byte0: UInt8, byte1: UInt8, byte2: UInt8, byte3: UInt8,
                                          byte4: UInt8, byte5: UInt8, byte6: UInt8, byte7: UInt8, byte8: UInt8, byte9: UInt8,
                                          byte10: UInt8, byte11: UInt8, byte12: UInt8, byte13: UInt8, byte14: UInt8,
                                          byte15: UInt8) -> CFUUIDRef;
    pub fn CFUUIDGetUUIDBytes(uuid: CFUUIDRef) -> CFUUIDBytes;
    pub fn CFUUIDCreateFromUUIDBytes(alloc: CFAllocatorRef, bytes: CFUUIDBytes) -> CFUUIDRef;
}

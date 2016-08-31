// Exports of <CoreFoundation/CFDictionary.h>

use libc::c_void;

use base::*;

pub type CFDictionaryRetainCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;
pub type CFDictionaryReleaseCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFDictionaryCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> CFStringRef;
pub type CFDictionaryEqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
pub type CFDictionaryHashCallBack = extern "C" fn(value: *const c_void) -> CFHashCode;

pub type CFDictionaryApplierFunction = extern "C" fn(key: *const c_void, value: *const c_void, context: *const c_void);

#[allow(non_snake_case)]
#[repr(C)]
pub struct CFDictionaryKeyCallBacks {
    pub version:         CFIndex,
    pub retain:          CFDictionaryRetainCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal:           CFDictionaryEqualCallBack,
    pub hash:            CFDictionaryHashCallBack
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct CFDictionaryValueCallBacks {
    pub version:         CFIndex,
    pub retain:          CFDictionaryRetainCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal:           CFDictionaryEqualCallBack
}

#[doc(hidden)]
#[repr(C)]
pub struct __CFDictionary {
    __private: c_void,
}

pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFDictionaryRef = *const __CFDictionary;

extern "C" {
    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFCopyStringDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;

    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    pub fn CFDictionaryGetTypeID() -> CFTypeID;

    pub fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: *const *const c_void, values: *const *const c_void, numValues: CFIndex, keyCallBacks: *const CFDictionaryKeyCallBacks, valueCallBacks: *const CFDictionaryValueCallBacks) -> CFDictionaryRef;
    pub fn CFDictionaryCreateCopy(allocator: CFAllocatorRef, theDict: CFDictionaryRef) -> CFDictionaryRef;
    pub fn CFDictionaryCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex, keysCallBacks: *const CFDictionaryKeyCallBacks, valueCallBacks: *const CFDictionaryValueCallBacks) -> CFMutableDictionaryRef;
    pub fn CFDictionaryCreateMutableCopy(allocator: CFAllocatorRef, capacity: CFIndex, theDict: CFDictionaryRef) -> CFMutableDictionaryRef;

    pub fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    pub fn CFDictionaryGetCountOfKey(theDict: CFDictionaryRef, value: *const c_void) -> CFIndex;
    pub fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *const c_void) -> Boolean;
    pub fn CFDictionaryContainsValue(theDict: CFDictionaryRef, value: *const c_void) -> Boolean;
    pub fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFDictionaryGetValueIfPresent(theDict: CFDictionaryRef, key: *const c_void, value: *mut *const c_void) -> Boolean;
    pub fn CFDictionaryGetKeysAndValues(theDict: CFDictionaryRef, keys: *mut *const c_void, values: *mut *const c_void);
    pub fn CFDictionaryApplyFunction(theDict: CFDictionaryRef, applier: CFDictionaryApplierFunction, context: *const c_void);

    pub fn CFDictionaryAddValue(theDict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    pub fn CFDictionarySetValue(theDict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    pub fn CFDictionaryReplaceValue(theDict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    pub fn CFDictionaryRemoveValue(theDict: CFMutableDictionaryRef, key: *const c_void);
    pub fn CFDictionaryRemoveAllValues(theDict: CFMutableDictionaryRef);
}

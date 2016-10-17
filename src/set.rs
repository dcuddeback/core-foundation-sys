// Exports of <CoreFoundation/CFSet.h>

use libc::c_void;
use ::{Boolean, CFAllocatorRef, CFHashCode, CFIndex, CFStringRef, CFTypeID};

#[doc(hidden)]
#[repr(C)]
pub struct __CFSet {
    __private: c_void
}

pub type CFSetRef = *const __CFSet;
pub type CFMutableSetRef = *mut __CFSet;

pub type CFSetApplierFunction = extern "C" fn(value: *const c_void, context: *mut c_void);
pub type CFSetCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> CFStringRef;
pub type CFSetEqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
pub type CFSetHashCallBack = extern "C" fn(value: *const c_void) -> CFHashCode;
pub type CFSetReleaseCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFSetRetainCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;

#[repr(C)]
pub struct CFSetCallBacks {
    pub version: CFIndex,
    pub retain: CFSetRetainCallBack,
    pub release: CFSetReleaseCallBack,
    pub copyDescription: CFSetCopyDescriptionCallBack,
    pub equal: CFSetEqualCallBack,
    pub hash: CFSetHashCallBack
}

extern {
    pub fn CFSetCreate(allocator: CFAllocatorRef, values: *mut *const c_void, numValues: CFIndex,
                       callBacks: *const CFSetCallBacks) -> CFSetRef;
    pub fn CFSetCreateCopy(allocator: CFAllocatorRef, theSet: CFSetRef) -> CFSetRef;

    pub fn CFSetCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex, callBacks: *const CFSetCallBacks)
                              -> CFMutableSetRef;
    pub fn CFSetCreateMutableCopy(allocator: CFAllocatorRef, capacity: CFIndex, theSet: CFSetRef) -> CFMutableSetRef;

    pub fn CFSetContainsValue(theSet: CFSetRef, value: *const c_void) -> Boolean;
    pub fn CFSetGetCount(theSet: CFSetRef) -> CFIndex;
    pub fn CFSetGetCountOfValue(theSet: CFSetRef, value: *const c_void) -> CFIndex;
    pub fn CFSetGetValue(theSet: CFSetRef, value: *const c_void) -> *const c_void;
    pub fn CFSetGetValueIfPresent(theSet: CFSetRef, candidate: *const c_void, value: *mut *const c_void) -> Boolean;
    pub fn CFSetGetValues(theSet: CFSetRef, values: *mut *const c_void);

    pub fn CFSetApplyFunction(theSet: CFSetRef, applier: CFSetApplierFunction, context: *mut c_void);

    pub fn CFSetGetTypeID() -> CFTypeID;

    pub fn CFSetAddValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetRemoveAllValues(theSet: CFMutableSetRef);
    pub fn CFSetRemoveValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetReplaceValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetSetValue(theSet: CFMutableSetRef, value: *const c_void);

    pub static kCFTypeSetCallBacks: CFSetCallBacks;
    pub static kCFCopyStringSetCallBacks: CFSetCallBacks;
}

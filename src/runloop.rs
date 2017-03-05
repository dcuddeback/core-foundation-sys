use libc::c_void;
use mach::port::mach_port_t;
use ::{Boolean, CFAbsoluteTime, CFAllocatorRef, CFAllocatorCopyDescriptionCallBack, CFAllocatorReleaseCallBack,
       CFAllocatorRetainCallBack, CFArrayRef, CFHashCode, CFIndex, CFOptionFlags, CFStringRef, CFTimeInterval,
       CFTypeID, CFTypeRef};

#[doc(hidden)]
#[repr(C)]
pub struct __CFRunLoop {
    __private: c_void
}

pub type CFRunLoopRef = *mut __CFRunLoop;

pub type CFRunLoopRunResult = i32;

pub const kCFRunLoopRunFinished: CFRunLoopRunResult = 1;
pub const kCFRunLoopRunStopped: CFRunLoopRunResult = 2;
pub const kCFRunLoopRunTimedOut: CFRunLoopRunResult = 3;
pub const kCFRunLoopRunHandledSource: CFRunLoopRunResult = 4;

extern "C" {
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub static kCFRunLoopDefaultMode: CFStringRef;

    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;

    pub fn CFRunLoopRun();
    pub fn CFRunLoopRunInMode(mode: CFStringRef, seconds: CFTimeInterval, returnAfterSourceHandled: Boolean)
                              -> CFRunLoopRunResult;

    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);
    pub fn CFRunLoopStop(rl: CFRunLoopRef);
    pub fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;

    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopContainsSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef) -> Boolean;
    pub fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);

    pub fn CFRunLoopAddObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    pub fn CFRunLoopContainsObserver(rl: CFRunLoopRef, source: CFRunLoopObserverRef, mode: CFStringRef) -> Boolean;
    pub fn CFRunLoopRemoveObserver(rl: CFRunLoopRef, source: CFRunLoopObserverRef, mode: CFStringRef);

    pub fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFStringRef);
    pub fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
    pub fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFStringRef;

    pub fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    pub fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFStringRef) -> CFAbsoluteTime;
    pub fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    pub fn CFRunLoopContainsTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef) -> Boolean;

    pub fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: *mut c_void);
    pub fn CFRunLoopGetTypeID() -> CFTypeID;
}

#[doc(hidden)]
#[repr(C)]
pub struct __CFRunLoopSource {
    __private: c_void
}

pub type CFRunLoopSourceRef = *mut __CFRunLoopSource;

#[repr(C)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
    pub equal: extern "C" fn(info1: *const c_void, info2: *const c_void) -> Boolean,
    pub hash: extern "C" fn(info: *const c_void) -> CFHashCode,
    pub schedule: extern "C" fn(info: *mut c_void, rl: CFRunLoopRef, mode: CFStringRef),
    pub cancel: extern "C" fn(info: *mut c_void, rl: CFRunLoopRef, mode: CFStringRef),
    pub perform: extern "C" fn(info: *mut c_void)
}

#[repr(C)]
pub struct CFRunLoopSourceContext1 {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
    pub equal: extern "C" fn(info1: *const c_void, info2: *const c_void) -> Boolean,
    pub hash: extern "C" fn(info: *const c_void) -> CFHashCode,
    pub getPort: extern "C" fn(info: *mut c_void) -> mach_port_t,
    pub perform: extern "C" fn(msg: *mut c_void, size: CFIndex, allocator: CFAllocatorRef,
                               info: *mut c_void) -> *mut c_void
}

extern "C" {
    pub fn CFRunLoopSourceCreate(allocator: CFAllocatorRef, order: CFIndex, context: *mut CFRunLoopSourceContext)
                                 -> CFRunLoopSourceRef;
    pub fn CFRunLoopSourceGetContext(source: CFRunLoopSourceRef, context: *mut CFRunLoopSourceContext);
    pub fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
    pub fn CFRunLoopSourceGetTypeID() -> CFTypeID;
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    pub fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
    pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);
}

#[doc(hidden)]
#[repr(C)]
pub struct __CFRunLoopObserver {
    __private: c_void
}

pub type CFRunLoopObserverRef = *mut __CFRunLoopObserver;

#[repr(C)]
pub struct CFRunLoopObserverContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
}

pub type CFRunLoopObserverCallBack = extern "C" fn(observer: CFRunLoopObserverRef, activity: CFRunLoopActivity,
                                                   info: *mut c_void);

pub type CFRunLoopActivity = CFOptionFlags;

pub const kCFRunLoopEntry: CFRunLoopActivity = (1 << 0);
pub const kCFRunLoopBeforeTimers: CFRunLoopActivity = (1 << 1);
pub const kCFRunLoopBeforeSources: CFRunLoopActivity = (1 << 2);
pub const kCFRunLoopBeforeWaiting: CFRunLoopActivity = (1 << 5);
pub const kCFRunLoopAfterWaiting: CFRunLoopActivity = (1 << 6);
pub const kCFRunLoopExit: CFRunLoopActivity = (1 << 7);
pub const kCFRunLoopAllActivities: CFRunLoopActivity = 0x0FFFFFFF;

extern "C" {
    pub fn CFRunLoopObserverCreateWithHandler(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean,
                                              order: CFIndex, block: *mut c_void) -> CFRunLoopObserverRef;
    pub fn CFRunLoopObserverCreate(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean,
                                   order: CFIndex, callout: CFRunLoopObserverCallBack,
                                   context: *mut CFRunLoopObserverContext) -> CFRunLoopObserverRef;

    pub fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
    pub fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
    pub fn CFRunLoopObserverGetContext(observer: CFRunLoopObserverRef, context: *mut CFRunLoopObserverContext);
    pub fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;

    pub fn CFRunLoopObserverGetTypeID() -> CFTypeID;

    pub fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
    pub fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
}

#[doc(hidden)]
#[repr(C)]
pub struct __CFRunLoopTimer {
    __private: c_void
}

pub type CFRunLoopTimerRef = *mut __CFRunLoopTimer;

pub type CFRunLoopTimerCallBack = extern "C" fn(timer: CFRunLoopTimerRef, info: *mut c_void);

#[repr(C)]
pub struct CFRunLoopTimerContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack
}

extern "C" {
    pub fn CFRunLoopTimerCreateWithHandler(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval,
                                           flags: CFOptionFlags, order: CFIndex, block: *mut c_void) -> CFRunLoopTimerRef;
    pub fn CFRunLoopTimerCreate(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval,
                                flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack,
                                context: *mut CFRunLoopTimerContext) -> CFRunLoopTimerRef;

    pub fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
    pub fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    pub fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
    pub fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
    pub fn CFRunLoopTimerGetTypeID() -> CFTypeID;
    pub fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
    pub fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
}

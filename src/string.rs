// Exports of <CoreFoundation/CFString.h>

use libc::{c_char};

use array::*;
use base::*;
use character_set::*;
use data::*;
use dictionary::*;
use locale::*;

pub type CFStringEncoding = u32;

pub const kCFStringEncodingMacRoman:      CFStringEncoding = 0;
pub const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
pub const kCFStringEncodingISOLatin1:     CFStringEncoding = 0x0201;
pub const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0B01;
pub const kCFStringEncodingASCII:         CFStringEncoding = 0x0600;
pub const kCFStringEncodingUnicode:       CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF8:          CFStringEncoding = 0x08000100;
pub const kCFStringEncodingNonLossyASCII: CFStringEncoding = 0x0BFF;
pub const kCFStringEncodingUTF16:         CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF16BE:       CFStringEncoding = 0x10000100;
pub const kCFStringEncodingUTF16LE:       CFStringEncoding = 0x14000100;
pub const kCFStringEncodingUTF32:         CFStringEncoding = 0x0c000100;
pub const kCFStringEncodingUTF32BE:       CFStringEncoding = 0x18000100;
pub const kCFStringEncodingUTF32LE:       CFStringEncoding = 0x1c000100;
pub const kCFStringEncodingInvalidId:     CFStringEncoding = 0xffffffff;

pub type CFStringCompareFlags = CFOptionFlags;
pub const kCFCompareCaseInsensitive:      CFStringCompareFlags = 1;
pub const kCFCompareBackwards:            CFStringCompareFlags = 4;
pub const kCFCompareAnchored:             CFStringCompareFlags = 8;
pub const kCFCompareNonliteral:           CFStringCompareFlags = 16;
pub const kCFCompareLocalized:            CFStringCompareFlags = 32;
pub const kCFCompareNumerically:          CFStringCompareFlags = 64;
pub const kCFCompareDiacriticInsensitive: CFStringCompareFlags = 128;
pub const kCFCompareWidthInsensitive:     CFStringCompareFlags = 256;
pub const kCFCompareForcedOrdering:       CFStringCompareFlags = 512;


extern "C" {
    pub fn CFStringGetTypeID() -> CFTypeID;

    // constructors

    pub fn CFStringCreateWithCString(alloc: CFAllocatorRef, cStr: *const c_char, encoding: CFStringEncoding) -> CFStringRef;
    pub fn CFStringCreateWithBytes(alloc: CFAllocatorRef, bytes: *const UInt8, numBytes: CFIndex, encoding: CFStringEncoding, isExternalRepresentation: Boolean) -> CFStringRef;
    pub fn CFStringCreateWithCharacters(alloc: CFAllocatorRef, chars: *const UniChar, numChars: CFIndex) -> CFStringRef;

    pub fn CFStringCreateWithCStringNoCopy(alloc: CFAllocatorRef, cStr: *const c_char, encoding: CFStringEncoding, contentsDeallocator: CFAllocatorRef) -> CFStringRef;
    pub fn CFStringCreateWithBytesNoCopy(alloc: CFAllocatorRef, bytes: *const UInt8, numBytes: CFIndex, encoding: CFStringEncoding, isExternalRepresentation: Boolean, contentsDeallocator: CFAllocatorRef) -> CFStringRef;
    pub fn CFStringCreateWithCharactersNoCopy(alloc: CFAllocatorRef, chars: *const UniChar, numChars: CFIndex, contentsDeallocator: CFAllocatorRef) -> CFStringRef;

    pub fn CFStringCreateWithSubstring(alloc: CFAllocatorRef, str: CFStringRef, range: CFRange) -> CFStringRef;
    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, theString: CFStringRef) -> CFStringRef;
    pub fn CFStringCreateWithFormat(alloc: CFAllocatorRef, formatOptions: CFDictionaryRef, format: CFStringRef, ...) -> CFStringRef;
    pub fn CFStringCreateMutable(alloc: CFAllocatorRef, maxLength: CFIndex) -> CFMutableStringRef;
    pub fn CFStringCreateMutableCopy(alloc: CFAllocatorRef, maxLength: CFIndex, theString: CFStringRef) -> CFMutableStringRef;
    pub fn CFStringCreateMutableWithExternalCharactersNoCopy(alloc: CFAllocatorRef, chars: *mut UniChar, numChars: CFIndex, capacity: CFIndex, externalCharactersAllocator: CFAllocatorRef) -> CFMutableStringRef;

    // accessors

    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    pub fn CFStringGetCharacterAtIndex(theString: CFStringRef, idx: CFIndex) -> UniChar;
    pub fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);

    // conversions

    pub fn CFStringGetCString(theString: CFStringRef, buffer: *mut c_char, bufferSize: CFIndex, encoding: CFStringEncoding) -> Boolean;
    pub fn CFStringGetCStringPtr(theString: CFStringRef, encoding: CFStringEncoding) -> *const c_char;
    pub fn CFStringGetCharactersPtr(theString: CFStringRef) -> *const UniChar;
    pub fn CFStringGetBytes(theString: CFStringRef, range: CFRange, encoding: CFStringEncoding, lossByte: UInt8, isExternalRepresentation: Boolean, buffer: *mut UInt8, maxBufLen: CFIndex, usedBufLen: *mut CFIndex) -> CFIndex;

    // string <-> data

    pub fn CFStringCreateFromExternalRepresentation(alloc: CFAllocatorRef, data: CFDataRef, encoding: CFStringEncoding) -> CFStringRef;
    pub fn CFStringCreateExternalRepresentation(alloc: CFAllocatorRef, theString: CFStringRef, encoding: CFStringEncoding, lossByte: UInt8) -> CFDataRef;

    pub fn CFStringGetSmallestEncoding(theString: CFStringRef) -> CFStringEncoding;
    pub fn CFStringGetFastestEncoding(theString: CFStringRef) -> CFStringEncoding;

    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;
    pub fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: CFStringEncoding) -> CFIndex;

    // filesystem path conversion

    pub fn CFStringGetFileSystemRepresentation(string: CFStringRef, buffer: *mut c_char, maxBufLen: CFIndex) -> Boolean;
    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: CFStringRef) -> CFIndex;
    pub fn CFStringCreateWithFileSystemRepresentation(alloc: CFAllocatorRef, buffer: *const c_char) -> CFStringRef;

    // comparison

    pub fn CFStringCompareWithOptionsAndLocale(theString1: CFStringRef, theString2: CFStringRef, rangeToCompare: CFRange, compareOptions: CFStringCompareFlags, locale: CFLocaleRef) -> CFComparisonResult;
    pub fn CFStringCompareWithOptions(theString1: CFStringRef, theString2: CFStringRef, rangeToCompare: CFRange, compareOptions: CFStringCompareFlags) -> CFComparisonResult;
    pub fn CFStringCompare(theString1: CFStringRef, theString2: CFStringRef, compareOptions: CFStringCompareFlags) -> CFComparisonResult;

    pub fn CFStringFindWithOptionsAndLocale(theString: CFStringRef, stringToFind: CFStringRef, rangeToSearch: CFRange, searchOptions: CFStringCompareFlags, locale: CFLocaleRef, result: *mut CFRange) -> Boolean;
    pub fn CFStringFindWithOptions(theString: CFStringRef, stringToFind: CFStringRef, rangeToSearch: CFRange, searchOptions: CFStringCompareFlags, result: *mut CFRange) -> Boolean;
    pub fn CFStringCreateArrayWithFindResults(alloc: CFAllocatorRef, theString: CFStringRef, stringToFind: CFStringRef, rangeToSearch: CFRange, compareOptions: CFStringCompareFlags) -> CFArrayRef;

    pub fn CFStringFind(theString: CFStringRef, stringToFind: CFStringRef, compareOptions: CFStringCompareFlags) -> CFRange;
    pub fn CFStringHasPrefix(theString: CFStringRef, prefix: CFStringRef) -> Boolean;
    pub fn CFStringHasSuffix(theString: CFStringRef, suffix: CFStringRef) -> Boolean;

    pub fn CFStringGetRangeOfComposedCharactersAtIndex(theString: CFStringRef, theIndex: CFIndex) -> CFRange;
    pub fn CFStringFindCharacterFromSet(theString: CFStringRef, theSet: CFCharacterSetRef, rangeToSearch: CFRange, searchOptions: CFStringCompareFlags, result: *mut CFRange) -> Boolean;
    pub fn CFStringGetLineBounds(theString: CFStringRef, range: CFRange, lineBeginIndex: *mut CFIndex, lineEndIndex: *mut CFIndex, contentsEndIndex: *mut CFIndex);
    pub fn CFStringGetParagraphBounds(string: CFStringRef, range: CFRange, parBeginIndex: *mut CFIndex, parEndIndex: *mut CFIndex, contentsEndIndex: *mut CFIndex);

    pub fn CFShow(obj: CFTypeRef);
    pub fn CFShowStr(str_: CFStringRef);
}

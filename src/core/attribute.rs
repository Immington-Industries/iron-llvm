use std::{mem, slice};
use std::ffi::{CString, CStr};

use libc::{c_char, c_int, c_uint};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;

use ::{LLVMRef, LLVMRefCtor};
use core::context::Context;

#[repr(C)]
pub enum AttributeType {
    LLVMZExtAttribute       = 1<<0u64,
    LLVMSExtAttribute       = 1<<1,
    LLVMNoReturnAttribute   = 1<<2,
    LLVMInRegAttribute      = 1<<3,
    LLVMStructRetAttribute  = 1<<4,
    LLVMNoUnwindAttribute   = 1<<5,
    LLVMNoAliasAttribute    = 1<<6,
    LLVMByValAttribute      = 1<<7,
    LLVMNestAttribute       = 1<<8,
    LLVMReadNoneAttribute   = 1<<9,
    LLVMReadOnlyAttribute   = 1<<10,
    LLVMNoInlineAttribute   = 1<<11,
    LLVMAlwaysInlineAttribute    = 1<<12,
    LLVMOptimizeForSizeAttribute = 1<<13,
    LLVMStackProtectAttribute    = 1<<14,
    LLVMStackProtectReqAttribute = 1<<15,
    LLVMAlignment = 31<<16,
    LLVMNoCaptureAttribute  = 1<<21,
    LLVMNoRedZoneAttribute  = 1<<22,
    LLVMNoImplicitFloatAttribute = 1<<23,
    LLVMNakedAttribute      = 1<<24,
    LLVMInlineHintAttribute = 1<<25,
    LLVMStackAlignment = 7<<26,
    LLVMReturnsTwice = 1 << 29,
    LLVMUWTable = 1 << 30,
    LLVMNonLazyBind = 1 << 31
}

pub trait AttributeCtor : LLVMRefCtor<LLVMAttributeRef> {
    fn new_enum(context: &Context, id: c_uint, val: AttributeType) -> LLVMAttributeRef {
        unsafe {
            LLVMCreateEnumAttribute(context.to_ref(), id, val as u64)
        }
    }

    fn new_string(context: LLVMContextRef, key: &str, value: &str) -> LLVMAttributeRef {
        let key_str = CString::new(key).unwrap();
        let key_str_len = key_str.as_bytes().len();
        let value_str = CString::new(value).unwrap();
        let value_str_len = value_str.as_bytes().len();
        unsafe {
            LLVMCreateStringAttribute(context,
                                      key_str.as_ptr(), key_str_len as c_uint,
                                      value_str.as_ptr(), value_str_len as c_uint)
        }
    }
}

impl LLVMRef<LLVMAttributeRef> for LLVMAttributeRef {
    fn to_ref(&self) -> LLVMAttributeRef {
        *self
    }
}

impl LLVMRefCtor<LLVMAttributeRef> for LLVMAttributeRef {
    unsafe fn from_ref(rf: LLVMAttributeRef) -> LLVMAttributeRef {
        rf
    }
}

pub trait Attribute : LLVMRef<LLVMAttributeRef> {
    fn is_enum(&self) -> bool {
        unsafe {
            LLVMIsEnumAttribute(self.to_ref()) > 0
        }
    }
    fn is_string(&self) -> bool {
        unsafe {
            LLVMIsStringAttribute(self.to_ref()) > 0
        }
    }
    fn get_enum_kind(&self) -> c_uint {
        unsafe {
            LLVMGetEnumAttributeKind(self.to_ref())
        }
    }
    fn get_enum_value(&self) -> Option<AttributeType> {
        unsafe {
            let value = LLVMGetEnumAttributeValue(self.to_ref());
            if value == 0 {
                None
            }
            else {
                Some(mem::transmute(value as u32))
            }
        }
    }

    #[allow(unused_mut)]
    fn get_string_attribute_kind(&self) -> String {
        let mut length: c_uint = 0;
        unsafe {
            let raw_chars = LLVMGetStringAttributeKind(self.to_ref(), length as *mut c_uint);
            let cstr = CStr::from_ptr(raw_chars);
            String::from_utf8_lossy(cstr.to_bytes()).into_owned()
        }
    }
}

impl Attribute for LLVMAttributeRef {}
impl AttributeCtor for LLVMAttributeRef {}

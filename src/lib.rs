// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(libc)]

extern crate libc;
#[macro_use] #[no_link] extern crate rustc_bitflags;

use core::context::*;

pub mod core;

mod llvmdeps;

#[test]
fn it_works() {
    let mut c1 = Context::new();

    let c = c1.get_ref();

    unsafe {
        let gc = ffi::LLVMGetGlobalContext();
    }
}

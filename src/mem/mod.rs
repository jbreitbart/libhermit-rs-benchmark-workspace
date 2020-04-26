// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library is derived from rlibc, not intended for general use,
//! and is superseded by a system libc if one is available. In a
//! freestanding context, however, common functions such as memset, memcpy,
//! etc are not implemented. This library provides an implementation of
//! these functions which are either required by libcore or called by rustc
//! implicitly.


#[no_mangle]
/// # Safety
pub unsafe extern "C" fn cpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	let mut i = 0;
	while i < n {
		*dest.add(i) = *src.add(i);
		i += 1;
	}
	dest
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn mve(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	if src < dest as *const u8 {
		// copy from end
		let mut i = n;
		while i != 0 {
			i -= 1;
			*dest.add(i) = *src.add(i);
		}
	} else {
		// copy from beginning
		let mut i = 0;
		while i < n {
			*dest.add(i) = *src.add(i);
			i += 1;
		}
	}
	dest
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn set(s: *mut u8, c: i32, n: usize) -> *mut u8 {
	let mut i = 0;
	while i < n {
		*s.add(i) = c as u8;
		i += 1;
	}
	s
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn cmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
	let mut i = 0;
	while i < n {
		let a = *s1.add(i);
		let b = *s2.add(i);
		if a != b {
			return a as i32 - b as i32;
		}
		i += 1;
	}
	0
}


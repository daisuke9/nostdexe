#![feature(lang_items)]
#![feature(start)]
#![feature(no_std)]
#![no_std]

extern crate libc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

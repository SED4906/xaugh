#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use std::{io::{Read, Write}, mem::size_of};

pub fn pad(s: usize) -> usize {
    (4 - (s % 4)) % 4
}

pub trait ReadStruct: Sized {
    fn read_struct(mut stream: impl Read) -> Self where Self: Sized, [(); size_of::<Self>()]: {
        let mut buf = [0u8;size_of::<Self>()];
        stream.read(&mut buf).unwrap();
        unsafe { std::mem::transmute_copy(&buf) }
    }
}

pub trait WriteStruct: Sized {
    fn write_struct(&self, mut stream: impl Write) -> () where Self: Sized, [(); size_of::<Self>()]: {
        let buf = unsafe {::core::slice::from_raw_parts(
            (self as *const Self) as *const u8,
            ::core::mem::size_of::<Self>(),
        )};
        stream.write(buf).unwrap();
    }
}
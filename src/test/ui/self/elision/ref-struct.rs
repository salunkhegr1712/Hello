// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

impl Struct {
    // Test using `&Struct` explicitly:

    fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
        f
        //[base]~^ ERROR lifetime mismatch
        //[nll]~^^ ERROR lifetime may not live long enough
    }

    fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
        f
        //[base]~^ ERROR lifetime mismatch
        //[nll]~^^ ERROR lifetime may not live long enough
    }

    fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
        f
        //[base]~^ ERROR lifetime mismatch
        //[nll]~^^ ERROR lifetime may not live long enough
    }

    fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
        f
        //[base]~^ ERROR lifetime mismatch
        //[nll]~^^ ERROR lifetime may not live long enough
    }

    fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
        f
        //[base]~^ ERROR lifetime mismatch
        //[nll]~^^ ERROR lifetime may not live long enough
    }
}

fn main() { }

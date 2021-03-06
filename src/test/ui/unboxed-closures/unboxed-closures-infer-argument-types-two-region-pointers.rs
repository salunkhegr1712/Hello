#![feature(fn_traits)]

// That a closure whose expected argument types include two distinct
// bound regions.

// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

use std::cell::Cell;

fn doit<T,F>(val: T, f: &F)
    where F : Fn(&Cell<&T>, &T)
{
    let x = Cell::new(&val);
    f.call((&x,&val))
}

pub fn main() {
    doit(0, &|x, y| {
        x.set(y);
        //[base]~^ ERROR E0312
        //[nll]~^^ lifetime may not live long enough
    });
}

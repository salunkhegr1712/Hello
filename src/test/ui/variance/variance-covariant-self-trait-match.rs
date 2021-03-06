#![allow(dead_code)]

// Test that even when `Self` is only used in covariant position, it
// is treated as invariant.

// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

trait Get {
    fn get() -> Self;
}

fn get_min_from_max<'min, 'max, G>()
    where 'max : 'min, G : 'max, &'max G : Get
{
    // Previously OK, now an error as traits are invariant.
    impls_get::<&'min G>();
    //[base]~^ ERROR mismatched types
    //[nll]~^^ ERROR lifetime may not live long enough
}

fn get_max_from_min<'min, 'max, G>()
    where 'max : 'min, G : 'max, &'min G : Get
{
    impls_get::<&'max G>();
    //[base]~^ ERROR mismatched types
    //[nll]~^^ ERROR lifetime may not live long enough
}

fn impls_get<G>() where G : Get { }

fn main() { }

// Test that dyn Foo<Bar = T> is invariant with respect to T.
// Failure to enforce invariance here can be weaponized, see #71550 for details.

// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

trait Foo {
    type Bar;
}

fn make() -> Box<dyn Foo<Bar = &'static u32>> {
    panic!()
}

fn take<'a>(_: &'a u32) {
    let _: Box<dyn Foo<Bar = &'a u32>> = make();
    //[base]~^ ERROR mismatched types [E0308]
    //[nll]~^^ ERROR lifetime may not live long enough
}

fn main() {}

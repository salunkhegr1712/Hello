#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn test<const N: usize>() -> [u8; N - 1] {
    //~^ ERROR evaluation of `test::<0_usize>::{constant#0}` failed
    todo!()
}

fn main() {
    test::<0>();
}

trait MyFn<Arg> {
    type Output;
    fn call(&self, arg: Arg) -> Self::Output;
}

struct Wrap<F>(F);

impl<A, B, F> MyFn<A> for Wrap<F>
where
    F: Fn(A) -> B
{
    type Output = B;

    fn call(&self, arg: A) -> Self::Output {
        (self.0)(arg)
    }
}


struct A;
fn test() -> impl for<'a> MyFn<&'a A, Output=impl Iterator + 'a> {
    //~^ ERROR implementation of `FnOnce` is not general enough
    Wrap(|a| Some(a).into_iter())
}

fn main() {}

/// Trait for executing closures on a box.
pub trait FnBox<A>{
    type Output;

    fn call_box(self: Box<Self>, args: A) -> Self::Output;
}

impl<A, F> FnBox<A> for F where F: FnOnce(A)
{
    type Output = F::Output;

    fn call_box(self: Box<F>, args: A) -> F::Output {
        (*self)(args)
    }
}


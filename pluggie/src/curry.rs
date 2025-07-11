use std::marker::{PhantomData, Tuple};
use tuplestructops::{TupleJoin, TupleSplit};

pub struct FuncWithArgs<F: Fn(Args) -> R, R, Args: Sized + Tuple> {
    pub f: F,
    pub marker: PhantomData<(Args, R)>,
}

impl<F: Fn(Args) -> R, R, Args: Sized + Tuple> Clone for FuncWithArgs<F, R, Args>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        FuncWithArgs {
            f: self.f.clone(),
            marker: PhantomData,
        }
    }
}

impl<F: Fn(Args) -> R, R, Args: Sized + Tuple> FnOnce<Args> for FuncWithArgs<F, R, Args> {
    type Output = R;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.f.call((args,))
    }
}

impl<F: Fn(Args) -> R, R, Args: Sized + Tuple> FnMut<Args> for FuncWithArgs<F, R, Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.f.call((args,))
    }
}

impl<F: Fn(Args) -> R, R, Args: Sized + Tuple> Fn<Args> for FuncWithArgs<F, R, Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.f.call((args,))
    }
}

pub struct FuncOnceWithArgs<F: FnOnce(Args) -> R, R, Args: Sized + Tuple> {
    pub f: F,
    pub marker: PhantomData<(Args, R)>,
}

impl<F: FnOnce(Args) -> R, R, Args: Sized + Tuple> FnOnce<Args> for FuncOnceWithArgs<F, R, Args> {
    type Output = R;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.f.call_once((args,))
    }
}

pub fn curry<
    'a,
    'r: 'a,
    R: 'static + Send + Sync,
    Args: Tuple + TupleSplit<(A,), Rest> + Send + Sync,
    F: Fn<Args, Output = R> + 'a + Send + Sync,
    A: 'a + Clone + Send + Sync,
    Rest: Tuple + 'r + Send + Sync,
>(
    f: F,
) -> impl FnOnce(A) -> Box<dyn Fn<Rest, Output = R> + 'a>
where
    (A,): TupleJoin<Rest, Output = Args>,
{
    move |a: A| {
        Box::new(FuncWithArgs {
            f: move |args: Rest| (&f).call_once((a.clone(),).join(args)),
            marker: PhantomData,
        })
    }
}

pub fn curry_once<
    'a,
    'r: 'a,
    R: 'static + Send + Sync,
    Args: Tuple + TupleSplit<(A,), Rest>,
    F: Fn<Args, Output = R> + 'a + Send + Sync,
    A: 'a + Send,
    Rest: Tuple + 'r + Send + Sync,
>(
    f: F,
) -> impl FnOnce(A) -> Box<dyn FnOnce<Rest, Output = R> + 'a + Send>
where
    (A,): TupleJoin<Rest, Output = Args>,
{
    move |a: A| {
        Box::new(FuncOnceWithArgs {
            f: move |args: Rest| (&f).call_once((a,).join(args)),
            marker: PhantomData,
        })
    }
}

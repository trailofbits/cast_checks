use std::{convert::Infallible, marker::PhantomData};

pub struct MaybeTryInto<T, U>(pub T, PhantomData<U>);

impl<T, U> MaybeTryInto<T, U> {
    pub fn new(operand: T) -> Self {
        Self(operand, PhantomData)
    }
}

impl<T, U> MaybeTryInto<T, U>
where
    T: TryInto<U>,
{
    /// If `expr: MaybeTryInto<T, U>` and `T: TryInto<U>`, then `expr.maybe_try_into()` resolves to
    /// this inherent method.
    pub fn maybe_try_into(self) -> Option<Result<U, <T as TryInto<U>>::Error>> {
        Some(<T as TryInto<U>>::try_into(self.0))
    }
}

pub trait MaybeTryIntoFallback<U> {
    /// If `expr: MaybeTryInto<T, U>` but not `T: TryInto<U>`, then `expr.maybe_try_into()` resolves
    /// to this trait method.
    fn maybe_try_into(self) -> Option<Result<U, Infallible>>;
}

impl<T, U> MaybeTryIntoFallback<U> for T {
    fn maybe_try_into(self) -> Option<Result<U, Infallible>> {
        None
    }
}

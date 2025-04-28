use std::{ffi::OsString, io::Stdin};

use crate::Error;

use aopt::{map::ErasedTy, opt::Pos, value::Infer};

type InnerVal<T> = Result<<T as Infer>::Val, <T as FieldVal>::Error>;

pub trait FieldVal
where
    Self: Infer,
{
    type Error: Into<Error>;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn map(val: InnerVal<Self>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(<Self as Infer>::infer_map(val?))
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! impl_value_for {
    ($type:ty) => {
        impl $crate::value::FieldVal for $type {
            type Error = $crate::Error;

            fn update(
                &mut self,
                val: Result<<Self as aopt::prelude::Infer>::Val, Self::Error>,
            ) -> Result<(), Self::Error> {
                *self = val?;
                Ok(())
            }
        }
    };
}

impl_value_for!(f32);
impl_value_for!(f64);
impl_value_for!(i8);
impl_value_for!(i16);
impl_value_for!(i32);
impl_value_for!(i64);
impl_value_for!(i128);
impl_value_for!(u8);
impl_value_for!(u16);
impl_value_for!(u32);
impl_value_for!(u64);
impl_value_for!(u128);
impl_value_for!(());
impl_value_for!(usize);
impl_value_for!(isize);
impl_value_for!(String);
impl_value_for!(OsString);
impl_value_for!(Stdin);

impl FieldVal for bool {
    type Error = Error;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(false)
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error> {
        *self = val?;
        Ok(())
    }
}

impl<T> FieldVal for Option<T>
where
    T: FieldVal<Error = Error>,
{
    type Error = Error;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(T::new())
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error> {
        *self = Some(T::map(val)?);
        Ok(())
    }
}

impl<T> FieldVal for Vec<T>
where
    T: FieldVal<Error = Error>,
{
    type Error = Error;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(vec![])
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error> {
        self.push(T::map(val)?);
        Ok(())
    }
}

impl<T, E> FieldVal for Result<T, E>
where
    E: From<Error>,
    T: FieldVal<Error = Error>,
{
    type Error = Error;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        T::new().map(Ok)
    }

    fn map(val: InnerVal<Self>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match val {
            Ok(val) => Ok(T::map(Ok(val)).map_err(Into::into)),
            Err(e) => Ok(Err(e.into())),
        }
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error> {
        match val {
            Ok(val) => {
                if let Ok(inner) = self.as_mut() {
                    inner.update(Ok(val))?;
                }
            }
            Err(e) => {
                *self = Err(e.into());
            }
        }
        Ok(())
    }
}

impl<T: FieldVal<Error = Error> + ErasedTy> FieldVal for Pos<T> {
    type Error = Error;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        T::new().map(|v| Pos::new(v))
    }

    fn update(&mut self, val: InnerVal<Self>) -> Result<(), Self::Error> {
        FieldVal::update(&mut self.0, val)
    }
}

use std::{ffi::OsString, io::Stdin};

use crate::Infer;

pub trait Field
where
    Self: Infer,
{
    type Error: Into<crate::Error>;

    fn new_value() -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn map_value(val: Result<<Self as Infer>::Val, Self::Error>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(<Self as Infer>::infer_map(val?))
    }

    fn set_value(
        &mut self,
        val: Result<<Self as Infer>::Val, Self::Error>,
    ) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! impl_field_for {
    ($type:ty) => {
        impl $crate::field::Field for $type {
            type Error = $crate::Error;

            fn set_value(
                &mut self,
                val: Result<<Self as $crate::Infer>::Val, Self::Error>,
            ) -> Result<(), Self::Error> {
                *self = val?;
                Ok(())
            }
        }
    };
}

impl_field_for!(f32);
impl_field_for!(f64);
impl_field_for!(i8);
impl_field_for!(i16);
impl_field_for!(i32);
impl_field_for!(i64);
impl_field_for!(i128);
impl_field_for!(u8);
impl_field_for!(u16);
impl_field_for!(u32);
impl_field_for!(u64);
impl_field_for!(u128);
impl_field_for!(());
impl_field_for!(usize);
impl_field_for!(isize);
impl_field_for!(String);
impl_field_for!(OsString);
impl_field_for!(Stdin);

impl Field for bool {
    type Error = crate::Error;

    fn new_value() -> Option<Self>
    where
        Self: Sized,
    {
        Some(false)
    }

    fn set_value(
        &mut self,
        val: Result<<Self as Infer>::Val, Self::Error>,
    ) -> Result<(), Self::Error> {
        *self = val?;
        Ok(())
    }
}

impl<T> Field for Option<T>
where
    T: Field<Error = crate::Error>,
{
    type Error = crate::Error;

    fn new_value() -> Option<Self>
    where
        Self: Sized,
    {
        Some(T::new_value())
    }

    fn set_value(
        &mut self,
        val: Result<<Self as Infer>::Val, Self::Error>,
    ) -> Result<(), Self::Error> {
        if self.is_none() {
            *self = Self::map_value(val)?;
        } else {
            self.as_mut().map(|v| v.set_value(val));
        }
        Ok(())
    }
}

impl<T> Field for Vec<T>
where
    T: Field<Error = crate::Error>,
{
    type Error = crate::Error;

    fn new_value() -> Option<Self>
    where
        Self: Sized,
    {
        Some(vec![])
    }

    fn set_value(
        &mut self,
        val: Result<<Self as Infer>::Val, Self::Error>,
    ) -> Result<(), Self::Error> {
        self.push(T::map_value(val)?);
        Ok(())
    }
}

impl<T, E> Field for Result<T, E>
where
    E: From<crate::Error>,
    T: Field<Error = crate::Error>,
{
    type Error = crate::Error;

    fn new_value() -> Option<Self>
    where
        Self: Sized,
    {
        T::new_value().map(Ok)
    }

    fn map_value(val: Result<<Self as Infer>::Val, Self::Error>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match val {
            Ok(val) => Ok(T::map_value(Ok(val)).map_err(Into::into)),
            Err(e) => Ok(Err(e.into())),
        }
    }

    fn set_value(
        &mut self,
        val: Result<<Self as Infer>::Val, Self::Error>,
    ) -> Result<(), Self::Error> {
        match val {
            Ok(val) => {
                if let Ok(inner) = self.as_mut() {
                    inner.set_value(Ok(val))?;
                }
            }
            Err(e) => {
                *self = Err(e.into());
            }
        }
        Ok(())
    }
}

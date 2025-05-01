use std::any::TypeId;
use std::ffi::OsString;
use std::io::Stdin;
use std::path::PathBuf;

use crate::Error;

use aopt::opt::Cmd;
use aopt::opt::Main;
use aopt::opt::Pos;
use aopt::prelude::Action;
use aopt::prelude::ConfigValue;
use aopt::prelude::Index;
use aopt::prelude::Style;
use aopt::value::Stop;

pub trait Infer: 'static {
    type Val;

    fn infer_act() -> Action {
        Action::Set
    }

    fn infer_force() -> bool {
        true
    }

    fn infer_ctor() -> String {
        aopt::set::ctor_default_name()
    }

    fn infer_index() -> Option<Index> {
        None
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Argument]
    }

    fn infer_ignore_name() -> bool {
        false
    }

    fn infer_ignore_alias() -> bool {
        false
    }

    fn infer_ignore_index() -> bool {
        true
    }

    fn infer_type_id() -> TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized;

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error>;

    fn infer_tweak_info<C>(_cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,

        C: ConfigValue + Default,
    {
        Ok(())
    }

    fn infer_fill_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        let act = Self::infer_act();
        let style = Self::infer_style();
        let index = Self::infer_index();
        let ignore_name = Self::infer_ignore_name();
        let ignore_alias = Self::infer_ignore_alias();
        let ignore_index = Self::infer_ignore_index();
        let force = Self::infer_force();
        let ctor = Self::infer_ctor();
        let type_id = Self::infer_type_id();

        Self::infer_tweak_info(cfg)?;
        (!cfg.has_ctor()).then(|| cfg.set_ctor(ctor));
        (!cfg.has_index()).then(|| index.map(|idx| cfg.set_index(idx)));
        (!cfg.has_type()).then(|| cfg.set_type_id(type_id));
        (!cfg.has_action()).then(|| cfg.set_action(act));
        (!cfg.has_style()).then(|| cfg.set_style(style));
        (!cfg.has_force()).then(|| cfg.set_force(force));
        (!cfg.has_action()).then(|| cfg.set_action(act));
        cfg.set_ignore_name(ignore_name);
        cfg.set_ignore_alias(ignore_alias);
        cfg.set_ignore_index(ignore_index);
        Ok(())
    }
}

impl Infer for bool {
    type Val = bool;

    fn infer_force() -> bool {
        false
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Combined, Style::Boolean]
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(false)
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        val
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        *self = val?;
        Ok(())
    }
}

impl Infer for Cmd {
    type Val = bool;

    fn infer_index() -> Option<Index> {
        Some(Index::forward(1))
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Cmd]
    }

    fn infer_ignore_index() -> bool {
        false
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Cmd(val?))
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        self.0 = val?;
        Ok(())
    }
}

impl<T: Infer + 'static> Infer for Pos<T> {
    type Val = <T as Infer>::Val;

    fn infer_style() -> Vec<Style> {
        vec![Style::Pos]
    }

    fn infer_ignore_name() -> bool {
        true
    }

    fn infer_ignore_alias() -> bool {
        true
    }

    fn infer_ignore_index() -> bool {
        false
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        T::infer_new().map(|v| Pos::new(v))
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Pos::new(<T as Infer>::infer_map(val)?))
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        Infer::infer_mut(&mut self.0, val)
    }
}

impl<T: Infer> Infer for Main<T> {
    type Val = <T as Infer>::Val;

    fn infer_act() -> Action {
        Action::Null
    }

    fn infer_index() -> Option<Index> {
        Some(Index::anywhere())
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Main]
    }

    fn infer_ignore_name() -> bool {
        true
    }

    fn infer_ignore_alias() -> bool {
        true
    }

    fn infer_ignore_index() -> bool {
        false
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        T::infer_new().map(|v| Main::new(v))
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Main::new(<T as Infer>::infer_map(val)?))
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        Infer::infer_mut(&mut self.0, val)
    }
}

impl Infer for Stdin {
    type Val = Stdin;

    fn infer_act() -> Action {
        Action::Set
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Boolean]
    }

    fn infer_ignore_alias() -> bool {
        true
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        val
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        *self = val?;
        Ok(())
    }

    /// For type Stdin, swap the name and default alias(`-`) when build configuration.
    fn infer_tweak_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        if let Some(name) = cfg.name() {
            cfg.add_alias(name.to_string());
        }
        cfg.set_name("-");
        Ok(())
    }
}

impl Infer for Stop {
    type Val = Stop;

    fn infer_act() -> Action {
        Action::Set
    }

    fn infer_style() -> Vec<Style> {
        vec![Style::Boolean]
    }

    fn infer_ignore_alias() -> bool {
        true
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        val
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        *self = val?;
        Ok(())
    }

    /// For type Stdin, swap the name and default alias(`-`) when build configuration.
    fn infer_tweak_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        if let Some(name) = cfg.name() {
            cfg.add_alias(name.to_string());
        }
        cfg.set_name("--");
        Ok(())
    }
}

impl<T: Infer> Infer for Option<T> {
    type Val = <T as Infer>::Val;

    fn infer_act() -> Action {
        <T as Infer>::infer_act()
    }

    fn infer_force() -> bool {
        false
    }

    fn infer_ctor() -> String {
        <T as Infer>::infer_ctor()
    }

    fn infer_index() -> Option<Index> {
        <T as Infer>::infer_index()
    }

    fn infer_style() -> Vec<Style> {
        <T as Infer>::infer_style()
    }

    fn infer_ignore_name() -> bool {
        <T as Infer>::infer_ignore_name()
    }

    fn infer_ignore_alias() -> bool {
        <T as Infer>::infer_ignore_alias()
    }

    fn infer_ignore_index() -> bool {
        <T as Infer>::infer_ignore_index()
    }

    fn infer_type_id() -> TypeId {
        <T as Infer>::infer_type_id()
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(T::infer_new())
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        T::infer_map(val).map(|v| Some(v))
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        if let Some(value) = self {
            value.infer_mut(val)?;
        } else {
            *self = Self::infer_map(val)?;
        }
        Ok(())
    }

    fn infer_tweak_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,

        C: ConfigValue + Default,
    {
        <T as Infer>::infer_tweak_info(cfg)
    }

    fn infer_fill_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        <T as Infer>::infer_fill_info(cfg)
    }
}

impl<T: Infer> Infer for Vec<T> {
    type Val = <T as Infer>::Val;

    fn infer_act() -> Action {
        Action::App
    }

    fn infer_force() -> bool {
        true
    }

    fn infer_ctor() -> String {
        <T as Infer>::infer_ctor()
    }

    fn infer_index() -> Option<Index> {
        <T as Infer>::infer_index()
    }

    fn infer_style() -> Vec<Style> {
        <T as Infer>::infer_style()
    }

    fn infer_ignore_name() -> bool {
        <T as Infer>::infer_ignore_name()
    }

    fn infer_ignore_alias() -> bool {
        <T as Infer>::infer_ignore_alias()
    }

    fn infer_ignore_index() -> bool {
        <T as Infer>::infer_ignore_index()
    }

    fn infer_type_id() -> TypeId {
        <T as Infer>::infer_type_id()
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        Some(vec![])
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(vec![T::infer_map(val)?])
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        self.push(T::infer_map(val)?);
        Ok(())
    }

    fn infer_tweak_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,

        C: ConfigValue + Default,
    {
        <T as Infer>::infer_tweak_info(cfg)
    }

    fn infer_fill_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        <T as Infer>::infer_fill_info(cfg)
    }
}

impl<T: Infer, E> Infer for Result<T, E>
where
    E: From<Error> + 'static,
{
    type Val = <T as Infer>::Val;

    fn infer_act() -> Action {
        <T as Infer>::infer_act()
    }

    fn infer_force() -> bool {
        false
    }

    fn infer_ctor() -> String {
        <T as Infer>::infer_ctor()
    }

    fn infer_index() -> Option<Index> {
        <T as Infer>::infer_index()
    }

    fn infer_style() -> Vec<Style> {
        <T as Infer>::infer_style()
    }

    fn infer_ignore_name() -> bool {
        <T as Infer>::infer_ignore_name()
    }

    fn infer_ignore_alias() -> bool {
        <T as Infer>::infer_ignore_alias()
    }

    fn infer_ignore_index() -> bool {
        <T as Infer>::infer_ignore_index()
    }

    fn infer_type_id() -> TypeId {
        <T as Infer>::infer_type_id()
    }

    fn infer_new() -> Option<Self>
    where
        Self: Sized,
    {
        T::infer_new().map(Ok)
    }

    fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match val {
            Ok(val) => Ok(T::infer_map(Ok(val)).map_err(Into::into)),
            Err(e) => Ok(Err(e.into())),
        }
    }

    fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
        match val {
            Ok(val) => {
                if let Ok(value) = self.as_mut() {
                    value.infer_mut(Ok(val))?;
                }
            }
            Err(e) => {
                *self = Err(e.into());
            }
        }
        Ok(())
    }

    fn infer_tweak_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,

        C: ConfigValue + Default,
    {
        <T as Infer>::infer_tweak_info(cfg)
    }

    fn infer_fill_info<C>(cfg: &mut C) -> Result<(), Error>
    where
        Self: Sized,
        C: ConfigValue + Default,
    {
        <T as Infer>::infer_fill_info(cfg)
    }
}

#[macro_export]
macro_rules! impl_value_for {
    ($type:ty) => {
        impl $crate::infer::Infer for $type {
            type Val = $type;

            fn infer_map(val: Result<Self::Val, Error>) -> Result<Self, Error>
            where
                Self: Sized,
            {
                val
            }

            fn infer_mut(&mut self, val: Result<Self::Val, Error>) -> Result<(), Error> {
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
impl_value_for!(PathBuf);
impl_value_for!(OsString);

// impl<T, E> Infer for Result<T, E>
// where
//     E: From<Error>,
//     T: Infer<Error = Error>,
// {
//     type Error = Error;

//     fn infer_new() -> Option<Self>
//     where
//         Self: Sized,
//     {
//         T::infer_new().map(Ok)
//     }

//     fn infer_map(val: InnerVal<Self>) -> Result<Self, Error>
//     where
//         Self: Sized,
//     {
//         match val {
//             Ok(val) => Ok(T::infer_map(Ok(val)).map_err(Into::into)),
//             Err(e) => Ok(Err(e.into())),
//         }
//     }

//     fn infer_mut(&mut self, val: InnerVal<Self>) -> Result<(), Error> {
//         match val {
//             Ok(val) => {
//                 if let Ok(inner) = self.as_mut() {
//                     inner.infer_mut(Ok(val))?;
//                 }
//             }
//             Err(e) => {
//                 *self = Err(e.into());
//             }
//         }
//         Ok(())
//     }
// }

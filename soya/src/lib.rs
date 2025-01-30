pub mod err;
pub mod field;
pub mod opt;

pub use aopt;
pub use aopt::Error;

use crate::opt::Opt;

use aopt::parser::DefaultSetChecker;
use aopt::prelude::AppServices;
use aopt::prelude::Args;
use aopt::prelude::OptConfig;
use aopt::prelude::PrefixOptValidator;
use aopt::prelude::StrParser;

pub use aopt::value::Infer;

pub type Creator = aopt::prelude::Creator<Opt, OptConfig, Error>;
pub type Set = aopt::prelude::OptSet<StrParser, Creator, PrefixOptValidator>;
pub type Ser = AppServices;
pub type Invoker<'a> = aopt::prelude::Invoker<'a, Set, Ser>;
pub type FwdPolicy = aopt::prelude::FwdPolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type PrePolicy = aopt::prelude::PrePolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type DelayPolicy = aopt::prelude::DelayPolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type OptSet<'a> = aopt::prelude::HCOptSet<Set, Invoker<'a>, Ser>;

pub mod prelude {
    pub use crate::field::Field;
    pub use crate::Creator;
    pub use crate::DelayPolicy;
    pub use crate::FwdPolicy;
    pub use crate::Invoker;
    pub use crate::OptSet;
    pub use crate::PrePolicy;
    pub use crate::Ser;
    pub use crate::Set;

    pub use aopt::args::Args;
    pub use aopt::ctx::Ctx;
    pub use aopt::ctx::NullStore;
    pub use aopt::prelude::Policy;
    pub use aopt::prelude::PolicyParser;
}

pub trait ParserImpl<'inv> {
    type Error: Into<crate::Error>;

    fn into_parser() -> Result<OptSet<'inv>, Self::Error> {
        let mut optset = OptSet::default();

        Self::update(&mut optset)?;
        Ok(optset)
    }

    fn update(optset: &mut OptSet<'inv>) -> Result<(), Self::Error>;

    fn parse(optset: OptSet<'inv>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Self::parse_args(Args::from_env(), optset)
    }

    fn parse_args(args: Args, optset: OptSet<'inv>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

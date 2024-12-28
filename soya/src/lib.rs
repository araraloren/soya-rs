pub mod err;
pub mod field;
pub mod opt;

pub use aopt;
pub use aopt::Error;

use crate::opt::Opt;

use aopt::parser::DefaultSetChecker;
use aopt::prelude::AppServices;
use aopt::prelude::OptConfig;
use aopt::prelude::OptSet;
use aopt::prelude::PrefixOptValidator;
use aopt::prelude::StrParser;

pub use aopt::value::Infer;

pub type Creator = aopt::prelude::Creator<Opt, OptConfig, Error>;
pub type Set = OptSet<StrParser, Creator, PrefixOptValidator>;
pub type Ser = AppServices;
pub type Invoker<'a> = aopt::prelude::Invoker<'a, Set, Ser>;
pub type FwdPolicy = aopt::prelude::FwdPolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type PrePolicy = aopt::prelude::PrePolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type DelayPolicy = aopt::prelude::DelayPolicy<Set, Ser, DefaultSetChecker<Set>>;
pub type FwdParser<'a> = aopt::prelude::Parser<'a, FwdPolicy>;
pub type PreParser<'a> = aopt::prelude::Parser<'a, PrePolicy>;
pub type DelayParser<'a> = aopt::prelude::Parser<'a, DelayPolicy>;

pub mod prelude {
    pub use crate::field::Field;
    pub use crate::Creator;
    pub use crate::DelayParser;
    pub use crate::DelayPolicy;
    pub use crate::FwdParser;
    pub use crate::FwdPolicy;
    pub use crate::Invoker;
    pub use crate::PreParser;
    pub use crate::PrePolicy;
    pub use crate::Ser;
    pub use crate::Set;

    pub use aopt::args::Args;
    pub use aopt::ctx::Ctx;
    pub use aopt::ctx::NullStore;
}

#[cfg(test)]
mod test {}

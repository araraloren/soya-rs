pub mod infer;
pub mod opt;

pub use aopt;

pub mod prelude {
    use crate::err::Error;
    use crate::opt::Opt;
    use aopt::parser::DefaultSetChecker;
    use aopt::prelude::OptConfig;
    use aopt::prelude::PrefixOptValidator;
    use aopt::prelude::StrParser;

    pub type Creator = aopt::prelude::Creator<Opt, OptConfig, Error>;
    pub type Set = aopt::prelude::OptSet<StrParser, Creator, PrefixOptValidator>;
    pub type Invoker<'inv> = aopt::prelude::Invoker<'inv, Set>;
    pub type OptSet<'inv> = aopt::prelude::HCOptSet<'inv, Set>;

    pub type FwdPolicy<'inv> =
        aopt::prelude::FwdPolicy<OptSet<'inv>, DefaultSetChecker<OptSet<'inv>>>;
    pub type PrePolicy<'inv> =
        aopt::prelude::PrePolicy<OptSet<'inv>, DefaultSetChecker<OptSet<'inv>>>;
    pub type DelayPolicy<'inv> =
        aopt::prelude::DelayPolicy<OptSet<'inv>, DefaultSetChecker<OptSet<'inv>>>;
    pub type SeqPolicy<'inv> =
        aopt::prelude::SeqPolicy<OptSet<'inv>, DefaultSetChecker<OptSet<'inv>>>;

    pub use aopt::ctx::NullStore;
    pub use aopt::prelude::Args;
    pub use aopt::prelude::Cmd;
    pub use aopt::prelude::Ctx;
    pub use aopt::prelude::Index;
    pub use aopt::prelude::Policy;
    pub use aopt::prelude::PolicyParser;
    pub use aopt::prelude::Pos;
    pub use aopt::prelude::SetCfg;
    pub use aopt::set::ctor_default_name;

    pub use crate::fetch_or_update;
    pub use crate::fetch_or_update_handler;
    pub use crate::infer::Infer;
    pub use crate::ParserImpl;
}

pub mod err {
    pub use aopt::raise_error as err;
    pub use aopt::raise_failure as fail;
    pub use aopt::Error;
}

pub mod _macro {
    #[macro_export]
    macro_rules! fetch_or_update {
        ($ctx:ident, $id:ident, $type:ty) => {
            let val = $ctx.value::<<$type as $crate::infer::Infer>::Val>();

            if let Some(value) = $id.as_mut() {
                <$type as $crate::infer::Infer>::infer_mut(value, val)?;
            } else {
                $id = Some(<$type as $crate::infer::Infer>::infer_map(val)?);
            }
        };
    }

    #[macro_export]
    macro_rules! fetch_or_update_handler {
        ($id:ident, $type:ty) => {
            |set, ctx| {
                let val = ctx.value::<<$type as $crate::infer::Infer>::Val>();

                if let Some(value) = $id.as_mut() {
                    <$type as $crate::infer::Infer>::infer_mut(value, val)?;
                } else {
                    $id = Some(<$type as $crate::infer::Infer>::infer_map(val)?);
                }
                Ok(Some(()))
            }
        };
    }
}

use crate::err::Error;
use aopt::{args::Args, parser::Policy, set::Set};

pub trait ParserImpl<'inv> {
    type Error: Into<Error>;
    type Parser<'a>: Set + Default
    where
        Self: 'a;
    type Policy<'a>: Policy<Error = Self::Error> + Default
    where
        Self: 'a;

    fn into_parser() -> Result<Self::Parser<'inv>, Self::Error> {
        let mut parser = <Self::Parser<'inv>>::default();

        Self::update(&mut parser)?;
        Ok(parser)
    }

    fn update(parser: &mut Self::Parser<'inv>) -> Result<(), Self::Error>;

    fn into_policy() -> Result<Self::Policy<'inv>, Self::Error> {
        let mut policy = <Self::Policy<'inv>>::default();

        Self::apply_settings(&mut policy)?;
        Ok(policy)
    }

    fn apply_settings(policy: &mut Self::Policy<'inv>) -> Result<(), Self::Error>;

    fn parse(args: Args) -> Result<Self, Self::Error>
    where
        Self: 'inv + Sized;

    fn parse_env() -> Result<Self, Self::Error>
    where
        Self: 'inv + Sized,
    {
        Self::parse(Args::from_env())
    }
}

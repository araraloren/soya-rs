use soya::aopt::opt::ConfigBuildInfer;
use soya::aopt::opt::ConfigValue;
use soya::aopt::set::Ctor;
use soya::aopt::set::Set;
use soya::aopt::set::SetExt;
use soya::err::err;
use soya::err::Error;
use soya::prelude::*;

#[derive(Debug)]
pub struct Git {
    // --debug
    debug: bool,

    // --cfg name=value
    cfg: Option<Vec<String>>,

    // clone
    clone: Option<Clone>,

    add: Option<Add>,
}

#[derive(Debug)]
pub struct Clone {
    // --depth u64
    depth: Option<u64>,

    // <repo>
    repo: String,

    // [dir]
    dir: Option<String>,
}

#[derive(Debug)]
pub struct Add {
    // <files>...
    files: Vec<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut values = Add::parse_env()?;

    dbg!(values);

    Ok(())
}

impl<'inv> ParserImpl<'inv> for Add {
    type Error = Error;

    type Parser<'a>
        = OptSet<'a>
    where
        Self: 'a;

    type Policy<'a>
        = FwdPolicy<'a>
    where
        Self: 'a;

    fn update(parser: &mut Self::Parser<'inv>) -> Result<(), Self::Error> {
        let ctor_default = ctor_default_name();
        let option_0 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("files");
                cfg.set_index(Index::range(Some(1), None));
                <Pos<Vec<String>> as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_1 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("-h");
                cfg.add_alias("--help");
                <bool as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };

        parser.insert(option_0);
        parser.insert(option_1);

        Ok(())
    }

    fn apply_settings(policy: &mut Self::Policy<'inv>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse(args: Args) -> Result<Self, Self::Error>
    where
        Self: 'inv + Sized,
    {
        let mut value_0 = <Pos<Vec<String>> as FieldVal>::new();
        let mut value_help = <bool as FieldVal>::new();
        let mut parser = <Self as ParserImpl<'_>>::into_parser()?;
        let mut policy = <Self as ParserImpl<'_>>::into_policy()?;

        parser
            .entry(0)?
            .on(|set, ctx| {
                let val = ctx.value::<<Pos<Vec<String>> as Infer>::Val>();

                if let Some(value_0) = value_0.as_mut() {
                    <Pos<Vec<String>> as FieldVal>::update(value_0, val)?;
                } else {
                    value_0 = Some(<Pos<Vec<String>> as FieldVal>::map(val)?);
                }
                Ok(Some(()))
            })
            .then(NullStore);
        parser
            .entry(1)?
            .on(|set, ctx| {
                let val = ctx.value::<<bool as Infer>::Val>();

                if let Some(value_help) = value_help.as_mut() {
                    <bool as FieldVal>::update(value_help, val)?;
                } else {
                    value_help = Some(<bool as FieldVal>::map(val)?);
                }
                Ok(Some(()))
            })
            .then(NullStore);

        let mut ret = parser.parse_policy(args, &mut policy)?;

        drop(parser);
        drop(policy);

        if value_help == Some(true) {
            // display help
            println!("display help for ..");
        }

        if let Some(error) = ret.take_failure() {
            Err(error)
        } else {
            Ok(Self {
                files: value_0
                    .ok_or_else(|| err!("Failed get value of field files"))
                    .map(|v| v.0)?,
            })
        }
    }
}

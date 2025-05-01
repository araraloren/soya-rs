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

    let values = Git::parse_env()?;

    println!("--> enable debug ? `{}`", values.debug);
    if let Some(cfg) = values.cfg {
        for str in cfg {
            if let Some((key, value)) = str.split_once('=') {
                println!("--> got a `{}` = `{}`", key, value);
            }
        }
    }
    if let Some(clone) = values.clone {
        println!("--> depth = `{}`", clone.depth.unwrap_or_default());
        println!("--> clone repo = `{}`", clone.repo);
        println!(
            "--> clone to `{}`",
            if let Some(dir) = clone.dir {
                dir
            } else {
                clone.repo
            }
        )
    } else if let Some(add) = values.add {
        for file in add.files {
            println!("--> add file `{}`", file);
        }
    }

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
                cfg.set_force(true);
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
                cfg.set_force(false);
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
        let mut value_0 = <Pos<Vec<String>> as Infer>::infer_new();
        let mut value_help = <bool as Infer>::infer_new();
        let mut parser = <Self as ParserImpl<'_>>::into_parser()?;
        let mut policy = <Self as ParserImpl<'_>>::into_policy()?;

        parser
            .entry(0)?
            .on(fetch_or_update_handler!(value_0, Pos<Vec<String>>))
            .then(NullStore);
        parser
            .entry(1)?
            .on(fetch_or_update_handler!(value_help, bool))
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

impl<'inv> ParserImpl<'inv> for Clone {
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

                cfg.set_name("--depth");
                cfg.set_force(false);
                <Option<u64> as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_1 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("repo");
                cfg.set_index(Index::Forward(1));
                cfg.set_force(true);
                <Pos<String> as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_2 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("dir");
                cfg.set_index(Index::Forward(2));
                cfg.set_force(false);
                <Pos<Option<String>> as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_3 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("-h");
                cfg.add_alias("--help");
                cfg.set_force(false);
                <bool as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };

        parser.insert(option_0);
        parser.insert(option_1);
        parser.insert(option_2);
        parser.insert(option_3);

        Ok(())
    }

    fn apply_settings(policy: &mut Self::Policy<'inv>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse(args: Args) -> Result<Self, Self::Error>
    where
        Self: 'inv + Sized,
    {
        let mut value_0 = <Option<u64> as Infer>::infer_new();
        let mut value_1 = <Pos<String> as Infer>::infer_new();
        let mut value_2 = <Pos<Option<String>> as Infer>::infer_new();
        let mut value_help = <bool as Infer>::infer_new();
        let mut parser = <Self as ParserImpl<'_>>::into_parser()?;
        let mut policy = <Self as ParserImpl<'_>>::into_policy()?;

        parser
            .entry(0)?
            .on(fetch_or_update_handler!(value_0, Option<u64>))
            .then(NullStore);
        parser
            .entry(1)?
            .on(fetch_or_update_handler!(value_1, Pos<String>))
            .then(NullStore);
        parser
            .entry(2)?
            .on(fetch_or_update_handler!(value_2, Pos<Option<String>>))
            .then(NullStore);
        parser
            .entry(3)?
            .on(fetch_or_update_handler!(value_help, bool))
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
                depth: value_0.ok_or_else(|| err!("Failed get value of field depth"))?,
                repo: value_1
                    .ok_or_else(|| err!("Failed get value of field repo"))
                    .map(|v| v.0)?,
                dir: value_2
                    .ok_or_else(|| err!("Failed get value of field dir"))
                    .map(|v| v.0)?,
            })
        }
    }
}

impl<'inv> ParserImpl<'inv> for Git {
    type Error = Error;

    type Parser<'a>
        = OptSet<'a>
    where
        Self: 'a;

    type Policy<'a>
        = PrePolicy<'a>
    where
        Self: 'a;

    fn update(parser: &mut Self::Parser<'inv>) -> Result<(), Self::Error> {
        let ctor_default = ctor_default_name();
        let option_0 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("--debug");
                cfg.set_force(false);
                <bool as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_1 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("--cfg");
                cfg.set_force(false);
                <Option<Vec<String>> as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_2 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("clone");
                <Cmd as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_3 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("add");
                <Cmd as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };
        let option_4 = {
            let cfg = {
                let mut cfg = SetCfg::<Self::Parser<'inv>>::default();

                cfg.set_name("-h");
                cfg.add_alias("--help");
                cfg.set_force(false);
                <bool as Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };

            parser.ctor_mut(&ctor_default)?.new_with(cfg)?
        };

        parser.insert(option_0);
        parser.insert(option_1);
        parser.insert(option_2);
        parser.insert(option_3);
        parser.insert(option_4);

        Ok(())
    }

    fn apply_settings(policy: &mut Self::Policy<'inv>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse(args: Args) -> Result<Self, Self::Error>
    where
        Self: 'inv + Sized,
    {
        let mut value_0 = <bool as Infer>::infer_new();
        let mut value_1 = <Option<Vec<String>> as Infer>::infer_new();
        let mut value_2 = None;
        let mut value_3 = None;
        let mut value_help = <bool as Infer>::infer_new();
        let mut parser = <Self as ParserImpl<'_>>::into_parser()?;
        let mut policy = <Self as ParserImpl<'_>>::into_policy()?;

        parser
            .entry(0)?
            .on(fetch_or_update_handler!(value_0, bool))
            .then(NullStore);
        parser
            .entry(1)?
            .on(fetch_or_update_handler!(value_1, Option<Vec<String>>))
            .then(NullStore);
        parser
            .entry(2)?
            .on(|set, ctx| {
                let index = ctx.idx()?;
                let mut args: Vec<_> = ctx.args().iter().map(|v| v.to_os_string()).collect();

                args.remove(index);
                value_2 = Clone::parse(Args::from(args)).ok();
                Ok(Some(()))
            })
            .then(NullStore);
        parser
            .entry(3)?
            .on(|set, ctx| {
                let index = ctx.idx()?;
                let mut args: Vec<_> = ctx.args().iter().map(|v| v.to_os_string()).collect();

                args.remove(index);
                value_3 = Add::parse(Args::from(args)).ok();
                Ok(Some(()))
            })
            .then(NullStore);
        parser
            .entry(4)?
            .on(fetch_or_update_handler!(value_help, bool))
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
                debug: value_0.ok_or_else(|| err!("Failed get value of field debug"))?,
                cfg: value_1.ok_or_else(|| err!("Failed get value of field cfg"))?,
                clone: value_2,
                add: value_3,
            })
        }
    }
}

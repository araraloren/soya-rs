use soya::err;
use soya::prelude::*;
use soya::Error;
use soya::Infer;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    #[derive(Debug)]
    pub struct Cli {
        a: bool,
        b: Option<i64>,
        c: String,
        d: Result<u64, Error>,
    }
    let mut field_0 = <bool as Field>::new_value();
    let mut field_1 = <Option<i64> as Field>::new_value();
    let mut field_2 = <String as Field>::new_value();
    let mut field_3 = <Result<u64, Error> as Field>::new_value();
    let mut parser = FwdParser::default();

    parser
        .add_opt("--opt=bool")?
        .on(|_: &mut Set, _: &mut Ser, ctx: &Ctx| {
            let val = ctx.value::<<bool as Infer>::Val>();

            if let Some(field_0) = field_0.as_mut() {
                <bool as Field>::set_value(field_0, val)?;
            } else {
                field_0 = Some(<bool as Field>::map_value(val)?);
            }
            Ok(Some(()))
        })?
        .then(NullStore);
    parser
        .add_opt("--cnt=int")?
        .on(|_: &mut Set, _: &mut Ser, ctx: &Ctx| {
            let val = ctx.value::<<Option<i64> as Infer>::Val>();

            if let Some(field_1) = field_1.as_mut() {
                <Option<i64> as Field>::set_value(field_1, val)?;
            } else {
                field_1 = Some(<Option<i64> as Field>::map_value(val)?);
            }
            Ok(Some(()))
        })?
        .then(NullStore);
    parser
        .add_opt("--win=string")?
        .on(|_: &mut Set, _: &mut Ser, ctx: &Ctx| {
            let val = ctx.value::<<String as Infer>::Val>();

            if let Some(field_2) = field_2.as_mut() {
                <String as Field>::set_value(field_2, val)?;
            } else {
                field_2 = Some(<String as Field>::map_value(val)?);
            }
            Ok(Some(()))
        })?
        .then(NullStore);
    parser
        .add_opt("--res=uint")?
        .on(|_: &mut Set, _: &mut Ser, ctx: &Ctx| {
            let val = ctx.value::<<Result<u64, Error> as Infer>::Val>();

            if let Some(field_3) = field_3.as_mut() {
                <Result<u64, Error> as Field>::set_value(field_3, val)?;
            } else {
                field_3 = Some(<Result<u64, Error> as Field>::map_value(val)?);
            }
            Ok(Some(()))
        })?
        .then(NullStore);
    parser.parse(Args::from_env())?.ok()?;

    dbg!(&parser);
    drop(parser);

    let cli = Cli {
        a: field_0.ok_or_else(|| err!("--opt is force required"))?,
        b: field_1.ok_or_else(|| err!("--cnt is force required"))?,
        c: field_2.ok_or_else(|| err!("--win is force required"))?,
        d: field_3.ok_or_else(|| err!("--res is force required"))?,
    };

    dbg!(cli);

    Ok(())
}

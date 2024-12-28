use std::any::TypeId;
use std::ffi::OsStr;

use crate::err;
use crate::Error;
use aopt::ctx::Ctx;
use aopt::opt::Action;
use aopt::opt::ConfigValue;
use aopt::opt::Help;
use aopt::opt::Index;
use aopt::opt::OptConfig;
use aopt::opt::Style;
use aopt::value::AnyValue;
use aopt::value::ErasedValue;
use aopt::value::ValAccessor;
use aopt::value::ValInitializer;
use aopt::value::ValStorer;
use aopt::Uid;

#[derive(Debug)]
pub struct Opt {
    uid: Uid,
    name: String,
    r#type: TypeId,
    help: Help,
    styles: Vec<Style>,
    index: Option<Index>,
    alias: Option<Vec<String>>,
    accessor: ValAccessor,
    action: Action,
    force: bool,
    matched: bool,
    ignore_name: bool,
    ignore_alias: bool,
    ignore_index: bool,
}

impl Opt {
    pub fn new(uid: Uid, name: String, r#type: TypeId, accessor: ValAccessor) -> Self {
        Self {
            uid,
            name,
            r#type,
            accessor,
            help: Default::default(),
            styles: Default::default(),
            index: Default::default(),
            alias: Default::default(),
            matched: Default::default(),
            force: Default::default(),
            action: Default::default(),
            ignore_name: Default::default(),
            ignore_alias: Default::default(),
            ignore_index: Default::default(),
        }
    }

    pub fn with_uid(mut self, uid: Uid) -> Self {
        self.uid = uid;
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_type(mut self, r#type: TypeId) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.help.set_hint(hint);
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help.set_help(help);
        self
    }

    pub fn with_opt_help(mut self, help: Help) -> Self {
        self.help = help;
        self
    }

    pub fn with_style(mut self, styles: Vec<Style>) -> Self {
        self.styles = styles;
        self
    }

    pub fn with_idx(mut self, index: Option<Index>) -> Self {
        self.index = index;
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn with_alias(mut self, alias: Option<Vec<String>>) -> Self {
        self.alias = alias;
        self
    }

    pub fn with_ignore_name(mut self, ignore_name: bool) -> Self {
        self.ignore_name = ignore_name;
        self
    }

    pub fn with_ignore_alias(mut self, ignore_alias: bool) -> Self {
        self.ignore_alias = ignore_alias;
        self
    }

    pub fn with_ignore_index(mut self, ignore_index: bool) -> Self {
        self.ignore_index = ignore_index;
        self
    }

    pub fn with_accessor(mut self, value: ValAccessor) -> Self {
        self.accessor = value;
        self
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    pub fn set_type(&mut self, r#type: TypeId) -> &mut Self {
        self.r#type = r#type;
        self
    }

    pub fn set_hint(&mut self, hint: impl Into<String>) -> &mut Self {
        self.help.set_hint(hint);
        self
    }

    pub fn set_help(&mut self, help: impl Into<String>) -> &mut Self {
        self.help.set_help(help);
        self
    }

    pub fn set_style(&mut self, styles: Vec<Style>) -> &mut Self {
        self.styles = styles;
        self
    }

    pub fn set_index(&mut self, index: Option<Index>) -> &mut Self {
        self.index = index;
        self
    }

    pub fn set_action(&mut self, action: Action) -> &mut Self {
        self.action = action;
        self
    }

    pub fn set_force(&mut self, force: bool) -> &mut Self {
        self.force = force;
        self
    }

    pub fn add_alias(&mut self, name: impl Into<String>) -> &mut Self {
        if let Some(alias) = &mut self.alias {
            alias.push(name.into());
        }
        self
    }

    pub fn rem_alias(&mut self, name: &str) -> &mut Self {
        if let Some(alias) = &mut self.alias {
            if let Some((i, _)) = alias.iter().enumerate().find(|(_, v)| v == &name) {
                alias.remove(i);
            }
        }
        self
    }

    pub fn set_accessor(&mut self, value: ValAccessor) -> &mut Self {
        self.accessor = value;
        self
    }
}

impl aopt::opt::Opt for Opt {
    fn reset(&mut self) {
        self.set_matched(false);
    }

    fn uid(&self) -> Uid {
        self.uid
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn r#type(&self) -> &TypeId {
        &self.r#type
    }

    fn hint(&self) -> &str {
        self.help.hint()
    }

    fn help(&self) -> &str {
        self.help.help()
    }

    fn valid(&self) -> bool {
        !self.force() || self.matched()
    }

    fn matched(&self) -> bool {
        self.matched
    }

    fn force(&self) -> bool {
        self.force
    }

    fn action(&self) -> &aopt::prelude::Action {
        &self.action
    }

    fn index(&self) -> Option<&Index> {
        self.index.as_ref()
    }

    fn alias(&self) -> Option<&Vec<String>> {
        self.alias.as_ref()
    }

    fn accessor(&self) -> &aopt::prelude::ValAccessor {
        &self.accessor
    }

    fn accessor_mut(&mut self) -> &mut aopt::prelude::ValAccessor {
        &mut self.accessor
    }

    fn ignore_alias(&self) -> bool {
        self.ignore_alias
    }

    fn ignore_name(&self) -> bool {
        self.ignore_name
    }

    fn ignore_index(&self) -> bool {
        self.ignore_index
    }

    fn set_uid(&mut self, uid: Uid) {
        self.uid = uid;
    }

    fn set_matched(&mut self, matched: bool) {
        self.matched = matched;
    }

    fn mat_style(&self, style: Style) -> bool {
        self.styles.iter().any(|v| v == &style)
    }

    fn mat_force(&self, force: bool) -> bool {
        self.force() == force
    }

    fn mat_name(&self, name: Option<&str>) -> bool {
        Some(self.name()) == name
    }

    fn mat_alias(&self, name: &str) -> bool {
        if let Some(alias) = &self.alias {
            alias.iter().any(|v| v == name)
        } else {
            false
        }
    }

    fn mat_index(&self, index: Option<(usize, usize)>) -> bool {
        if let Some((index, total)) = index {
            if let Some(realindex) = self.index() {
                if let Some(realindex) = realindex.calc_index(index, total) {
                    return realindex == index;
                }
            }
        }
        false
    }

    fn init(&mut self) -> Result<(), aopt::Error> {
        self.accessor.initialize()
    }
}

fn gen_hint(
    hint: Option<impl Into<String>>,
    n: &str,
    idx: Option<&Index>,
    alias: Option<&Vec<String>>,
) -> String {
    let hint_generator = || {
        let mut names = Vec::with_capacity(1 + alias.map(|v| v.len()).unwrap_or_default());

        // add name
        names.push(n);
        // add alias
        if let Some(alias_vec) = alias {
            for alias in alias_vec {
                names.push(alias.as_str());
            }
        }
        // sort name by len
        names.sort_by_key(|v| v.len());
        if let Some(index) = idx {
            let index_string = index.to_help();

            // add index string
            if index_string.is_empty() {
                names.join(", ")
            } else {
                format!("{}@{}", names.join(", "), index_string)
            }
        } else {
            names.join(", ")
        }
    };

    hint.map(|v| v.into()).unwrap_or_else(hint_generator)
}

impl TryFrom<OptConfig> for Opt {
    type Error = Error;

    fn try_from(mut value: OptConfig) -> Result<Self, Self::Error> {
        let r#type = value.take_type();
        let name = value.take_name();
        let force = value.take_force();
        let index = value.take_index();
        let alias = value.take_alias();
        let hint = value.take_hint();
        let help = value.take_help();
        let action = value.take_action();
        let storer = value.take_storer();
        let styles = value.take_style();
        let initializer = value.take_initializer();
        let ignore_name = value.ignore_name();
        let ignore_alias = value.ignore_alias();
        let ignore_index = value.ignore_index();

        let action = action.unwrap_or(Action::App);
        let force = force.unwrap_or(false);
        let styles = styles.ok_or_else(|| err!("incomplete configuration: missing Style"))?;
        let name = name.ok_or_else(|| err!("incomplete configuration: missing option name"))?;
        let hint = gen_hint(hint.as_ref(), &name, index.as_ref(), alias.as_ref());
        let help = help.unwrap_or_default();
        let r#type =
            r#type.ok_or_else(|| err!("incomplete configuration: missing option value type"))?;
        let help = Help::default().with_help(help).with_hint(hint);

        if storer.is_some() {
            tracing::warn!("The `storer` of OptConfig `{name}` will be ignored by soya option")
        }
        if initializer.is_some() {
            tracing::warn!("The `initializer` of OptConfig `{name}` will be ignored by soya option")
        }

        let storer = ValStorer::new(Box::new(
            |_: Option<&OsStr>, _: &Ctx, _: &Action, _: &mut AnyValue| Ok(()),
        ));
        let initializer = ValInitializer::fallback();

        if ignore_alias {
            if let Some(alias) = &alias {
                debug_assert!(
                    !alias.is_empty(),
                    "option {} not support alias: {:?}",
                    name,
                    alias
                );
            }
        }
        if ignore_index {
            if let Some(index) = &index {
                debug_assert!(
                    !index.is_null(),
                    "please remove the index, option `{}` not support positional parameters: {:?}",
                    name,
                    index
                );
            }
        } else {
            debug_assert!(
                    index.is_some(),
                    "please provide an index, indicate the position you want to capture for option `{}`.",
                    name
                );
        }
        Ok(
            Opt::new(0, name, r#type, ValAccessor::new(storer, initializer))
                .with_force(force)
                .with_idx(index)
                .with_action(action)
                .with_alias(alias)
                .with_style(styles)
                .with_opt_help(help)
                .with_ignore_name(ignore_name)
                .with_ignore_alias(ignore_alias)
                .with_ignore_index(ignore_index),
        )
    }
}

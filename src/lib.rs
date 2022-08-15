use std::collections::HashMap;

use crate::{defaults::*, parse::*};

mod class;
mod consts;
mod defaults;
mod media_queries;
mod modifiers;
mod parse;

#[cfg(test)]
mod tests;

#[cfg(feature = "inventory")]
#[macro_use]
pub mod inventory;

/// used to generate css out of classes
///
/// contains shorthands and replacements that can be modified
/// to customize the css generation
#[derive(Default)]
pub struct Zephyr {
    /// for non-value classes
    pub declarations: HashMap<String, String>,
    /// special declarations. Fn(Value) -> declarations
    pub specials: HashMap<String, SpecialDeclaration>,

    /// list of property short-hands
    pub properties: HashMap<String, String>,
    /// list of value short-hands
    pub values: HashMap<String, String>,
    /// list of pseudo-class short-hands
    pub modifiers: HashMap<String, String>,
    /// list of pseudo-element short-hands
    pub pseudos: HashMap<String, String>,
}

/// value -> declarations
pub type SpecialDeclaration = Box<dyn Fn(&str) -> String>;

#[derive(PartialEq, Debug)]
pub enum ZephyrError {
    /// the provided rule has invalid braces (single braces, or in incorrect order `..}...{..`)
    InvalidBraces,
    /// the provided rule isn't a no-variable rule, but no variables were provided
    ValueMissing,
}

impl Zephyr {
    /// generates css rules for all the of the classes that parse correctly
    pub fn generate_classes<'a>(&self, classes: impl IntoIterator<Item = &'a str>) -> String {
        // TODO we could return (css, seen_classes)
        let mut seen_classes = vec![];

        let span = tracing::trace_span!("generating classes");
        let _enter = span.enter();

        let classes = classes
            .into_iter()
            // get a list with one class per element
            .flat_map(|s| s.split_ascii_whitespace())
            .map(|s| s.trim())
            // remove duplicates
            // we use this instead of a HashSet because we want it to not change the order
            // if it's a performance concern, we could use HashSet on normal builds and the filter for test
            // but i don't really like that
            .filter(|s| {
                if seen_classes.contains(s) {
                    false
                } else {
                    seen_classes.push(s);
                    true
                }
            })
            // we ignore errors
            // TODO change this to call parse_class directly
            // TODO then group by media query
            .flat_map(|c| match self.generate_class(c) {
                Ok(v) => Some(v),
                Err(err) => {
                    // trace error
                    tracing::error!("error generating {c}: {err:?}");
                    None
                }
            })
            .collect::<Vec<_>>();

        let len = classes.len();
        tracing::trace!("finished generating {len} classes");

        classes.join("")
    }

    /// this one returns an error if parsing or generating fails
    pub fn generate_class(&self, class: &str) -> Result<String, ZephyrError> {
        let c = parse_class(class)?;
        c.generate_with_media_query(self)
    }

    /// builds a `Zephyr` with the default ruleset
    pub fn new() -> Self {
        Self {
            declarations: default_declarations(),
            properties: default_properties(),
            values: default_values(),
            modifiers: default_modifiers(),
            pseudos: default_pseudos(),
            specials: default_specials(),
        }
    }

    pub fn with_css_colors(mut self) -> Self {
        self.declarations.extend(
            crate::consts::CSS_COLORS
                .iter()
                .map(|c| (c.to_string(), format!("color:{c}"))),
        );
        self
    }
}

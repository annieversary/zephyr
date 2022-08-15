use std::collections::HashMap;

use defaults::default_modifiers;

use crate::{defaults::*, parse::*};

mod class;
mod consts;
mod defaults;
mod media_queries;
mod modifiers;
mod parse;

#[cfg(feature = "inventory")]
#[macro_use]
pub mod inventory;

/// used to generate css out of classes
///
/// contains shorthands and replacements that can be modified
/// to customize the css generation
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

    /// builds a `Zephyr` without the default ruleset
    pub fn new_without_defaults() -> Self {
        Self {
            declarations: HashMap::new(),
            properties: HashMap::new(),
            values: HashMap::new(),
            modifiers: HashMap::new(),
            pseudos: HashMap::new(),
            specials: HashMap::new(),
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use class::Class;

    #[test]
    fn generate_margin_works() {
        let z = Zephyr::new();

        let class = Class {
            property: "m",
            value: Some("1rem"),
            modifiers: vec![].into(),
            pseudo: None,
            original: "m[1rem]",
            value_type: class::ValueType::Normal,
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(css, r#".m\[1rem\]{margin:1rem;}"#);

        let class = Class {
            property: "m",
            value: Some("1rem"),
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
            value_type: class::ValueType::Normal,
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(css, r#".m\[1rem\]focus:focus{margin:1rem;}"#);

        let class = Class {
            property: "m",
            value: Some("1rem"),
            modifiers: vec!["focus", "hover", "odd"].into(),
            pseudo: None,
            original: "m[1rem]focus,hover,odd",
            value_type: class::ValueType::Normal,
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(
            css,
            r#".m\[1rem\]focus,hover,odd:focus:hover:nth-child\(odd\){margin:1rem;}"#
        );
    }

    #[test]
    fn generate_classes_works() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["flex-row"]);
        assert_eq!(classes, r#".flex-row{display:flex;flex-direction:row}"#);

        let classes = z.generate_classes(["m[3rem]hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".m\[3rem\]hover,focus\$placeholder:hover:focus::placeholder{margin:3rem;}"#
        );

        let classes = z.generate_classes(["flex|hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".flex\|hover,focus\$placeholder:hover:focus::placeholder{display:flex}"#
        );

        let classes = z.generate_classes(["mr[0.5rem]"]);
        assert_eq!(classes, r#".mr\[0\.5rem\]{margin-right:0.5rem;}"#);
    }

    #[test]
    fn generate_multiple_works() {
        let z = Zephyr::new();

        let classes_joined = z.generate_classes(["flex-row mt[1rem]"]);
        let classes_separate = z.generate_classes(["flex-row", "mt[1rem]"]);
        assert_eq!(
            classes_joined,
            r#".flex-row{display:flex;flex-direction:row}.mt\[1rem\]{margin-top:1rem;}"#
        );
        assert_eq!(classes_separate, classes_joined);
    }

    #[test]
    fn generate_specials_works() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["mx[1rem]"]);
        assert_eq!(
            classes,
            r#".mx\[1rem\]{margin-left:1rem;margin-right:1rem;}"#
        );
    }

    #[test]
    fn generate_with_spaces_works() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["border[1px_solid_black]"]);
        assert_eq!(
            classes,
            r#".border\[1px_solid_black\]{border:1px solid black;}"#
        );
    }

    #[test]
    fn generate_literals_works() {
        let z = Zephyr::new();

        // the curly brackets indicate that the value should not go through replacements
        let classes = z.generate_classes(["border{1px_solid_black}", "w{full}"]);
        assert_eq!(
            classes,
            r#".border\{1px_solid_black\}{border:1px_solid_black;}.w\{full\}{width:full;}"#
        );
    }

    #[test]
    fn generate_with_media_query() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["m[1rem]sm"]);
        assert_eq!(
            classes,
            r#"@media(min-width:640px){.m\[1rem\]sm{margin:1rem;}}"#
        );
    }

    #[test]
    fn generate_variable() {
        let z = Zephyr::new();

        // the parens indicate that it should be replaced by `var(--...)`
        let classes = z.generate_classes(["m(my-margin)"]);
        assert_eq!(classes, r#".m\(my-margin\){margin:var(--my-margin);}"#);
    }

    #[test]
    fn generate_css_colors() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["white blanchedalmond"]);
        assert_eq!(
            classes,
            r#".white{color:white}.blanchedalmond{color:blanchedalmond}"#
        );
    }
}

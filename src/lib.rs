use std::collections::HashMap;

use defaults::default_modifiers;

use crate::{defaults::*, parse::*};

mod class;
mod defaults;
mod parse;

#[cfg(feature = "inventory")]
#[macro_use]
pub mod inventory;

pub struct Zephyr {
    /// for non-value classes
    pub rules: HashMap<String, String>,
    /// special rules. Fn(Value) -> Properties
    pub specials: HashMap<String, SpecialRule>,

    /// list of name short-hands
    pub names: HashMap<String, String>,
    /// list of value short-hands
    pub values: HashMap<String, String>,
    /// list of modifier short-hands
    pub modifiers: HashMap<String, String>,
    /// list of pseudo-element short-hands
    pub pseudos: HashMap<String, String>,
}

/// Value -> Rules
pub type SpecialRule = Box<dyn Fn(&str) -> String>;

impl Zephyr {
    /// builds a `Zephyr` with the default ruleset
    pub fn new() -> Self {
        Self {
            rules: default_rules(),
            names: default_names(),
            values: default_values(),
            modifiers: default_modifiers(),
            pseudos: default_pseudos(),
            specials: default_specials(),
        }
    }

    /// builds a `Zephyr` without the default ruleset
    pub fn new_without_defaults() -> Self {
        Self {
            rules: HashMap::new(),
            names: HashMap::new(),
            values: HashMap::new(),
            modifiers: HashMap::new(),
            pseudos: HashMap::new(),
            specials: HashMap::new(),
        }
    }

    pub fn generate_classes<'a>(&self, classes: impl IntoIterator<Item = &'a str>) -> String {
        // TODO when we have media queries, we can do something to group them by the query, and then emit those together

        // TODO we could return (css, seen_classes)
        let mut seen_classes = vec![];

        classes
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
            .flat_map(|c| self.generate_class(c).ok().flatten())
            .collect::<Vec<_>>()
            .join("")
    }

    /// this one returns an error if parsing or generating fails
    // TODO add an error type
    pub fn generate_class(&self, class: &str) -> Result<Option<String>, &'static str> {
        parse_class(class).map(|c| c.generate(self)).transpose()
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
            name: "m",
            value: Some("1rem"),
            modifiers: vec![].into(),
            pseudo: None,
            original: "m[1rem]",
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(css, r#".m\[1rem\] { margin: 1rem; }"#);

        let class = Class {
            name: "m",
            value: Some("1rem"),
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(css, r#".m\[1rem\]focus:focus { margin: 1rem; }"#);

        let class = Class {
            name: "m",
            value: Some("1rem"),
            modifiers: vec!["focus", "hover", "odd"].into(),
            pseudo: None,
            original: "m[1rem]focus,hover,odd",
        };
        let css = class.generate(&z).unwrap();
        assert_eq!(
            css,
            r#".m\[1rem\]focus,hover,odd:focus:hover:nth-child\(odd\) { margin: 1rem; }"#
        );
    }

    #[test]
    fn generate_classes_works() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["flex-row"]);
        assert_eq!(
            classes,
            r#".flex-row { display: flex; flex-direction: row; }"#
        );

        let classes = z.generate_classes(["m[3rem]hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".m\[3rem\]hover,focus\$placeholder:hover:focus::placeholder { margin: 3rem; }"#
        );

        let classes = z.generate_classes(["flex|hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".flex\|hover,focus\$placeholder:hover:focus::placeholder { display: flex; }"#
        );

        let classes = z.generate_classes(["mr[0.5rem]"]);
        assert_eq!(classes, r#".mr\[0\.5rem\] { margin-right: 0.5rem; }"#);
    }

    #[test]
    fn generate_multiple_works() {
        let z = Zephyr::new();

        let classes_joined = z.generate_classes(["flex-row mt[1rem]"]);
        let classes_separate = z.generate_classes(["flex-row", "mt[1rem]"]);
        assert_eq!(
            classes_joined,
            r#".flex-row { display: flex; flex-direction: row; }.mt\[1rem\] { margin-top: 1rem; }"#
        );
        assert_eq!(classes_separate, classes_joined);
    }

    #[test]
    fn generate_specials_works() {
        let z = Zephyr::new();

        let classes = z.generate_classes(["mx[1rem]"]);
        assert_eq!(
            classes,
            r#".mx\[1rem\] { margin-left: 1rem; margin-right: 1rem; }"#
        );
    }
}

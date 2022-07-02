use std::collections::HashMap;

use defaults::default_modifiers;

use crate::{defaults::*, parse::*};

mod class;
mod defaults;
mod parse;

pub struct Zephyr {
    /// for non-value rules
    pub rules: HashMap<String, String>,

    pub names: HashMap<String, String>,
    pub values: HashMap<String, String>,
    pub modifiers: HashMap<String, String>,
    pub pseudos: HashMap<String, String>,
}

impl Zephyr {
    /// builds a `Zephyr` with the default ruleset
    pub fn new() -> Self {
        Self {
            rules: default_rules(),
            names: default_names(),
            values: default_values(),
            modifiers: default_modifiers(),
            pseudos: default_pseudos(),
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
        }
    }

    pub fn generate_css(&self, classes: &[&str]) -> String {
        classes
            .into_iter()
            .flat_map(|c| self.generate_class(c))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn generate_class(&self, class: &str) -> Option<String> {
        parse_class(class).map(|c| c.generate(self))
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
        let css = class.generate(&z);
        assert_eq!(css, r#".m\[1rem\] { margin: 1rem; }"#);

        let class = Class {
            name: "m",
            value: Some("1rem"),
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
        };
        let css = class.generate(&z);
        assert_eq!(css, r#".m\[1rem\]focus:focus { margin: 1rem; }"#);

        let class = Class {
            name: "m",
            value: Some("1rem"),
            modifiers: vec!["focus", "hover", "odd"].into(),
            pseudo: None,
            original: "m[1rem]focus,hover,odd",
        };
        let css = class.generate(&z);
        assert_eq!(
            css,
            r#".m\[1rem\]focus,hover,odd:focus:hover:nth-child\(odd\) { margin: 1rem; }"#
        );
    }

    #[test]
    fn generate_classes_works() {
        let z = Zephyr::new();

        let classes = z.generate_css(&["flex-row"]);
        assert_eq!(
            classes,
            r#".flex-row { display: flex; flex-direction: row; }"#
        );

        let classes = z.generate_css(&["m[3rem]hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".m\[3rem\]hover,focus\$placeholder:hover:focus::placeholder { margin: 3rem; }"#
        );

        let classes = z.generate_css(&["flex|hover,focus$placeholder"]);
        assert_eq!(
            classes,
            r#".flex\|hover,focus\$placeholder:hover:focus::placeholder { display: flex; }"#
        );
    }
}

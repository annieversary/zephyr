use std::collections::HashMap;

use defaults::default_rules;

use crate::parse::*;

mod class;
mod defaults;
mod modifiers;
mod name;
mod parse;

// pub fn generate_and_write(classes: &[&str], path: impl AsRef<Path>) -> Result<(), std::io::Error> {
//     let out = generate_css(classes);
//     std::fs::write(path, out)?;

//     Ok(())
// }

pub struct Zephyr {
    pub rules: HashMap<String, String>,
}

impl Zephyr {
    /// builds a `Zephyr` with the default ruleset
    pub fn new() -> Self {
        Self {
            rules: default_rules(),
        }
    }

    /// builds a `Zephyr` without the default ruleset
    pub fn new_without_rules() -> Self {
        Self {
            rules: HashMap::new(),
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
            name: "m".into(),
            value: Some("1rem"),
            modifiers: vec![].into(),
            pseudo: None,
            original: "m[1rem]",
        };
        let css = class.generate(&z);
        assert_eq!(css, r#".m\[1rem\] { margin: 1rem; }"#);

        let class = Class {
            name: "m".into(),
            value: Some("1rem"),
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
        };
        let css = class.generate(&z);
        assert_eq!(css, r#".m\[1rem\]focus:focus { margin: 1rem; }"#);

        let class = Class {
            name: "m".into(),
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

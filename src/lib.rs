use once_cell::sync::Lazy;
use std::{collections::HashMap, path::Path};
use thiserror::Error;

use crate::parse::*;

mod modifiers;
mod parse;

pub fn generate(classes: &[&str], path: impl AsRef<Path>) -> Result<(), Error> {
    let out = generate_css(classes);
    std::fs::write(path, out)?;

    Ok(())
}

pub fn generate_css(classes: &[&str]) -> String {
    classes
        .into_iter()
        .flat_map(|c| generate_class(c))
        .collect::<Vec<_>>()
        .join("")
}

pub fn generate_class(class: &str) -> Option<String> {
    let class = parse_class(class)?;
    let rule = RULES.get(&class.name)?;
    Some(rule.generate(&class))
}

static RULES: Lazy<HashMap<&str, &dyn Rule>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("m", &Margin as &dyn Rule);
    m
});

trait Rule: Sync {
    fn generate<'a>(&self, class: &Class<'a>) -> String;
}

impl Rule for Margin {
    fn generate<'a>(&self, class: &Class<'a>) -> String {
        format!(
            "{selector} {{ margin: {value}; }}",
            selector = class.selector(),
            value = class.value
        )
    }
}
struct Margin;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Disconnect(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_margin_works() {
        let class = Class {
            name: "m",
            value: "1rem",
            modifiers: vec![].into(),
            pseudo: None,
            original: "m[1rem]",
        };
        let css = Margin.generate(&class);
        assert_eq!(css, ".m[1rem] { margin: 1rem; }");

        let class = Class {
            name: "m",
            value: "1rem",
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
        };
        let css = Margin.generate(&class);
        assert_eq!(css, ".m[1rem]focus:focus { margin: 1rem; }");

        let class = Class {
            name: "m",
            value: "1rem",
            modifiers: vec!["focus", "hover", "odd"].into(),
            pseudo: None,
            original: "m[1rem]focus,hover,odd",
        };
        let css = Margin.generate(&class);
        assert_eq!(
            css,
            ".m[1rem]focus,hover,odd:focus:hover:nth-child(odd) { margin: 1rem; }"
        );
    }

    #[test]
    fn generate_classes_works() {
        let classes = generate_css(&["m[3rem]hover,focus$placeholder"]);

        assert_eq!(
            classes,
            ".m[3rem]hover,focus$placeholder:hover:focus::placeholder { margin: 3rem; }"
        );
    }
}

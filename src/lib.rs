use crate::{parse::*, rules::*};
use std::path::Path;

mod modifiers;
mod name;
mod parse;
mod rules;

pub fn generate_and_write(classes: &[&str], path: impl AsRef<Path>) -> Result<(), std::io::Error> {
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
    let rule = RULES
        .get(&class.name.as_str())
        .unwrap_or(&(&General as &dyn Rule));
    Some(rule.generate(&class))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_margin_works() {
        let class = Class {
            name: "m".into(),
            value: "1rem",
            modifiers: vec![].into(),
            pseudo: None,
            original: "m[1rem]",
        };
        let css = General.generate(&class);
        assert_eq!(css, r#".m\[1rem\] { margin: 1rem; }"#);

        let class = Class {
            name: "m".into(),
            value: "1rem",
            modifiers: vec!["focus"].into(),
            pseudo: None,
            original: "m[1rem]focus",
        };
        let css = General.generate(&class);
        assert_eq!(css, r#".m\[1rem\]focus:focus { margin: 1rem; }"#);

        let class = Class {
            name: "m".into(),
            value: "1rem",
            modifiers: vec!["focus", "hover", "odd"].into(),
            pseudo: None,
            original: "m[1rem]focus,hover,odd",
        };
        let css = General.generate(&class);
        assert_eq!(
            css,
            r#".m\[1rem\]focus,hover,odd:focus:hover:nth-child\(odd\) { margin: 1rem; }"#
        );
    }

    #[test]
    fn generate_classes_works() {
        let classes = generate_css(&["m[3rem]hover,focus$placeholder"]);

        assert_eq!(
            classes,
            r#".m\[3rem\]hover,focus\$placeholder:hover:focus::placeholder { margin: 3rem; }"#
        );
    }
}

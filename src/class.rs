use crate::{modifiers::Modifiers, name::Name, Zephyr};

#[derive(PartialEq, Debug)]
pub(crate) struct Class<'a> {
    pub name: Name<'a>,
    pub value: Option<&'a str>,
    pub modifiers: Modifiers<'a>,
    pub pseudo: Option<&'a str>,
    /// the original unparsed value
    /// needed to generate the css selector
    pub original: &'a str,
}

impl<'a> Class<'a> {
    pub(crate) fn selector(&self) -> String {
        let Class {
            modifiers,
            pseudo,
            original,
            ..
        } = self;

        let mut rest = if let Some(mods) = modifiers.get() {
            format!(":{mods}")
        } else {
            "".to_string()
        };
        if let Some(pseudo) = pseudo {
            rest.push_str("::");
            rest.push_str(pseudo);
        }

        format!(".{original}{rest}")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('|', "\\|")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('#', "\\#")
            .replace('$', "\\$")
            .replace('\'', "\\'")
            .replace('*', "\\*")
            .replace('%', "\\%")
    }

    /// TODO return result
    pub(crate) fn generate(&self, z: &Zephyr) -> String {
        let name = self.name.as_str();
        if let Some(val) = self.value {
            let selector = self.selector();
            format!("{selector} {{ {name}: {val}; }}",)
        } else if let Some(v) = z.rules.get(name) {
            let selector = self.selector();
            format!("{selector} {{ {v} }}",)
        } else {
            panic!("{name} is not a no-variable rule, and no variables were provided");
        }
    }
}

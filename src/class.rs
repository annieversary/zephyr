use crate::Zephyr;

#[derive(PartialEq, Debug)]
pub(crate) struct Class<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub modifiers: Vec<&'a str>,
    pub pseudo: Option<&'a str>,
    /// the original unparsed value
    /// needed to generate the css selector
    pub original: &'a str,
}

impl<'a> Class<'a> {
    pub(crate) fn selector(&self, z: &Zephyr) -> String {
        let Class {
            modifiers,
            pseudo,
            original,
            ..
        } = self;

        let mut rest = modifiers
            .iter()
            .map(|m| -> &str { z.modifiers.get(*m).map(AsRef::as_ref).unwrap_or(m) })
            .collect::<Vec<_>>()
            .join(":");

        if let Some(pseudo) = pseudo {
            rest.push_str("::");
            let pseudo: &str = z.pseudos.get(*pseudo).map(AsRef::as_ref).unwrap_or(pseudo);
            rest.push_str(pseudo);
        }

        // TODO we can probably skip the format here, we just need to push the char at the start
        if !rest.is_empty() {
            rest.insert(0, ':')
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

    pub(crate) fn generate(&self, z: &Zephyr) -> Result<String, &'static str> {
        let name = z
            .names
            .get(self.name)
            .map(AsRef::as_ref)
            .unwrap_or(self.name);
        let selector = self.selector(z);

        if let Some(val) = self.value {
            let val = z.values.get(val).map(AsRef::as_ref).unwrap_or(val);
            Ok(format!("{selector} {{ {name}: {val}; }}",))
        } else if let Some(v) = z.rules.get(name) {
            Ok(format!("{selector} {{ {v} }}",))
        } else {
            Err("{name} is not a no-variable rule, and no variables were provided")
        }
    }
}

use crate::{
    media_queries::{ReducedMotion, Responsive},
    modifiers::Modifiers,
    Zephyr, ZephyrError,
};

#[derive(PartialEq, Debug)]
pub(crate) struct Class<'a> {
    pub property: &'a str,
    pub value: Option<&'a str>,
    /// if true, no replacements will be done on `value`
    pub value_literal: bool,
    pub modifiers: Modifiers<'a>,
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
            .all
            .iter()
            .filter(|m| Responsive::from_str(*m).is_none() && ReducedMotion::from_str(*m).is_none())
            .map(|m| -> &str { z.modifiers.get(*m).map(AsRef::as_ref).unwrap_or(m) })
            .collect::<Vec<_>>()
            .join(":");

        if let Some(pseudo) = pseudo {
            rest.push_str("::");
            let pseudo: &str = z.pseudos.get(*pseudo).map(AsRef::as_ref).unwrap_or(pseudo);
            rest.push_str(pseudo);
        }

        if !rest.is_empty() {
            rest.insert(0, ':')
        }

        let mut r = format!("{original}{rest}")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('|', "\\|")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('.', "\\.")
            .replace('#', "\\#")
            .replace('$', "\\$")
            .replace('\'', "\\'")
            .replace('*', "\\*")
            .replace('%', "\\%");
        r.insert(0, '.');
        r
    }

    /// generates the css rule for this class
    /// does not generate the corresponding media query
    pub(crate) fn generate(&self, z: &Zephyr) -> Result<String, ZephyrError> {
        let property = z
            .properties
            .get(self.property)
            .map(AsRef::as_ref)
            .unwrap_or(self.property);
        let selector = self.selector(z);

        if let Some(val) = self.value {
            let val = if self.value_literal {
                val.to_string()
            } else {
                z.values
                    .get(val)
                    .map(AsRef::as_ref)
                    .unwrap_or(val)
                    .replace('_', " ")
            };

            if let Some(fun) = z.specials.get(property) {
                let v = fun(&val);
                Ok(format!("{selector}{{{v}}}",))
            } else {
                Ok(format!("{selector}{{{property}:{val};}}"))
            }
        } else if let Some(v) = z.declarations.get(property) {
            Ok(format!("{selector}{{{v}}}"))
        } else {
            Err(ZephyrError::ValueMissing)
        }
    }

    pub fn generate_with_media_query(&self, z: &Zephyr) -> Result<String, ZephyrError> {
        let mut css = self.generate(z)?;

        if let Some(r) = &self.modifiers.responsive {
            css = r.wrap(&css);
        }
        if let Some(r) = &self.modifiers.reduced_motion {
            css = r.wrap(&css);
        }

        Ok(css)
    }
}

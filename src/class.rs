use std::borrow::Cow;

use crate::{
    indent,
    media_queries::{wrap_in_query, ReducedMotion, Responsive},
    modifiers::Modifiers,
    nl, space, Zephyr, ZephyrError,
};

#[derive(PartialEq, Debug)]
pub(crate) struct Class<'a> {
    pub property: &'a str,
    pub value: Option<&'a str>,
    pub value_type: ValueType,
    pub modifiers: Modifiers<'a>,
    pub pseudo: Option<&'a str>,
    /// the original unparsed value
    /// needed to generate the css selector
    pub original: &'a str,
}

#[derive(PartialEq, Debug)]
pub(crate) enum ValueType {
    /// replacements will be performed
    ///
    /// eg: `m[1rem]`
    Normal,
    /// no replacements will be done. value will be output as-is
    ///
    /// eg: `border{1px solid black}`
    Literal,
    /// value will be output as `var(--value)`, without any replacements
    ///
    /// eg: `c(main-color)`
    Variable,
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
            .replace('<', "\\<")
            .replace('@', "\\@")
            .replace('%', "\\%");
        r.insert(0, '.');
        r
    }

    /// generates the css rule for this class
    /// does not generate the corresponding media query
    pub(crate) fn generate(&self, z: &Zephyr, indent_level: usize) -> Result<String, ZephyrError> {
        let property = z
            .properties
            .get(self.property)
            .map(AsRef::as_ref)
            .unwrap_or(self.property);
        let selector = self.selector(z);

        let space = space(z.pretty_print);
        let indent2 = indent(z.pretty_print, indent_level + 1);
        let indent = indent(z.pretty_print, indent_level);
        let nl = nl(z.pretty_print);

        if let Some(val) = self.value {
            let val = match self.value_type {
                ValueType::Normal => {
                    let v = z
                        .context_aware_values
                        .get(property)
                        .and_then(|h| h.get(val))
                        .or_else(|| z.values.get(val))
                        .map(AsRef::as_ref)
                        .unwrap_or(val);
                    replace_underscores(v)
                }
                ValueType::Literal => val.into(),
                ValueType::Variable => format!("var(--{val})").into(),
            };

            if let Some(fun) = z.specials.get(property) {
                let v = fun(&val);
                Ok(format!(
                    "{indent}{selector}{space}{{{nl}{indent2}{v}{nl}{indent}}}{nl}",
                ))
            } else {
                Ok(format!(
                    "{indent}{selector}{space}{{{nl}{indent2}{property}:{space}{val}{nl}{indent}}}{nl}"
                ))
            }
        } else if let Some(v) = z.declarations.get(property) {
            Ok(format!(
                "{indent}{selector}{space}{{{nl}{indent2}{v}{nl}{indent}}}{nl}"
            ))
        } else {
            Err(ZephyrError::ValueMissing)
        }
    }

    pub fn generate_with_media_query(&self, z: &Zephyr) -> Result<String, ZephyrError> {
        let mut queries: Vec<String> = vec![];
        if let Some(r) = &self.modifiers.responsive {
            queries.extend(r.queries());
        }
        if let Some(r) = &self.modifiers.reduced_motion {
            queries.extend(r.queries().iter().map(ToString::to_string));
        }

        let css = self.generate(z, queries.is_empty().then_some(0).unwrap_or(1))?;

        Ok(wrap_in_query(css, &queries, z.pretty_print))
    }
}

/// replaces underscores with spaces
fn replace_underscores(s: &str) -> Cow<str> {
    if s.contains('_') {
        s.replace('_', " ").into()
    } else {
        s.into()
    }
}

use crate::modifiers::Modifiers;

pub(crate) fn parse_class<'a>(original: &'a str) -> Option<Class<'a>> {
    let (class, pseudo) = if let Some((class, pseudo)) = original.split_once('$') {
        (class, Some(pseudo))
    } else {
        (original, None)
    };

    let start = pos(class, '[')?;
    let end = pos(class, ']')?;

    if start > end {
        return None;
    }

    let mods = if end + 1 == class.len() {
        vec![]
    } else {
        class[end + 1..].split(',').collect()
    };

    Some(Class {
        name: &class[0..start],
        value: &class[start + 1..end],
        modifiers: mods.into(),
        pseudo,
        original,
    })
}

#[derive(PartialEq, Debug)]
pub(crate) struct Class<'a> {
    pub name: &'a str,
    pub value: &'a str,
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
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('#', "\\#")
            .replace('$', "\\$")
            .replace('\'', "\\'")
            .replace('*', "\\*")
    }
}

fn pos(s: &str, c: char) -> Option<usize> {
    s.find(|v| v == c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(class: &str, (name, value, modifiers, pseudo): (&str, &str, Vec<&str>, Option<&str>)) {
        assert_eq!(
            parse_class(class),
            Some(Class {
                name,
                value,
                modifiers: modifiers.into(),
                pseudo,
                original: class
            })
        );
    }

    #[test]
    fn parse_works() {
        check("m[1rem]", ("m", "1rem", vec![], None));
        check("text-align[center]", ("text-align", "center", vec![], None));
        check("something[one:two]", ("something", "one:two", vec![], None));
        // testing out weird unicode stuffs
        check("he🥰llo[one:two]", ("he🥰llo", "one:two", vec![], None));
    }

    #[test]
    fn parse_modifier() {
        check("a[b]hover", ("a", "b", vec!["hover"], None));
        check(
            "text-align[center]focus",
            ("text-align", "center", vec!["focus"], None),
        );
    }

    #[test]
    fn parse_multiple_modifiers() {
        check(
            "a[b]hover,focus,odd",
            ("a", "b", vec!["hover", "focus", "odd"], None),
        );
    }

    #[test]
    fn parse_pseudo() {
        check(
            "a[b]hover,focus,odd$before",
            ("a", "b", vec!["hover", "focus", "odd"], Some("before")),
        );
        check(
            "a[b]hover$before$after",
            ("a", "b", vec!["hover"], Some("before$after")),
        );
    }

    #[test]
    fn out_of_order_is_none() {
        assert_eq!(parse_class("a]b["), None);
        assert_eq!(parse_class("a]b[c]"), None);
    }
}

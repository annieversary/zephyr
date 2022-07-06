use crate::{class::Class, ZephyrError};

pub(crate) fn parse_class<'a>(original: &'a str) -> Result<Class<'a>, ZephyrError> {
    // this code is kinda repetitive but idk

    let (class, pseudo) = if let Some((class, pseudo)) = original.split_once('$') {
        (class, Some(pseudo))
    } else {
        (original, None)
    };

    if let Some(p) = pos(class, '|') {
        let mods = if p + 1 == class.len() {
            vec![]
        } else {
            class[p + 1..].split(',').collect()
        };

        return Ok(Class {
            name: &class[0..p],
            value: None,
            modifiers: mods.into(),
            pseudo,
            original,
            value_literal: false,
        });
    }

    match (pos(class, '{'), pos(class, '}')) {
        (Some(start), Some(end)) if start <= end => {
            let mods = if end + 1 == class.len() {
                vec![]
            } else {
                class[end + 1..].split(',').collect()
            };

            return Ok(Class {
                name: &class[0..start],
                value: Some(&class[start + 1..end]),
                modifiers: mods.into(),
                pseudo,
                original,
                value_literal: true,
            });
        }
        // go to [...] case
        (None, None) => {}
        // braces do not form a valid block
        _ => {
            return Err(ZephyrError::InvalidBraces);
        }
    };

    match (pos(class, '['), pos(class, ']')) {
        (Some(start), Some(end)) if start <= end => {
            let mods = if end + 1 == class.len() {
                vec![]
            } else {
                class[end + 1..].split(',').collect()
            };

            return Ok(Class {
                name: &class[0..start],
                value: Some(&class[start + 1..end]),
                modifiers: mods.into(),
                pseudo,
                original,
                value_literal: false,
            });
        }
        _ => {
            return Ok(Class {
                name: &class[0..],
                value: None,
                modifiers: vec![].into(),
                pseudo,
                original,
                value_literal: false,
            });
        }
    }
}

fn pos(s: &str, c: char) -> Option<usize> {
    s.find(|v| v == c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(
        class: &str,
        (name, value, modifiers, pseudo): (&str, Option<&str>, Vec<&str>, Option<&str>),
    ) {
        assert_eq!(
            parse_class(class),
            Ok(Class {
                name,
                value,
                modifiers: modifiers.into(),
                pseudo,
                original: class,
                value_literal: false,
            })
        );
    }
    fn check_literal(
        class: &str,
        (name, value, modifiers, pseudo): (&str, Option<&str>, Vec<&str>, Option<&str>),
    ) {
        assert_eq!(
            parse_class(class),
            Ok(Class {
                name,
                value,
                modifiers: modifiers.into(),
                pseudo,
                original: class,
                value_literal: true,
            })
        );
    }

    #[test]
    fn parse_works() {
        check("m[1rem]", ("m", Some("1rem"), vec![], None));
        check(
            "text-align[center]",
            ("text-align", Some("center"), vec![], None),
        );
        check(
            "something[one:two]",
            ("something", Some("one:two"), vec![], None),
        );
        // testing out weird unicode stuffs
        check(
            "he🥰llo[one:two]",
            ("he🥰llo", Some("one:two"), vec![], None),
        );
    }

    #[test]
    fn parse_literal_values() {
        // testing out weird unicode stuffs
        check_literal(
            "hello{hey_hello}",
            ("hello", Some("hey_hello"), vec![], None),
        );
    }

    #[test]
    fn parse_modifier() {
        check("a[b]hover", ("a", Some("b"), vec!["hover"], None));
        check(
            "text-align[center]focus",
            ("text-align", Some("center"), vec!["focus"], None),
        );
    }

    #[test]
    fn parse_multiple_modifiers() {
        check(
            "a[b]hover,focus,odd",
            ("a", Some("b"), vec!["hover", "focus", "odd"], None),
        );
    }

    #[test]
    fn parse_pseudo() {
        check(
            "a[b]hover,focus,odd$before",
            (
                "a",
                Some("b"),
                vec!["hover", "focus", "odd"],
                Some("before"),
            ),
        );
        check(
            "a[b]hover$before$after",
            ("a", Some("b"), vec!["hover"], Some("before$after")),
        );
    }

    #[test]
    fn closing_before_opening_means_no_value() {
        check("a]b[", ("a]b[", None, vec![], None));
        check("a]b[c]", ("a]b[c]", None, vec![], None));
    }

    #[test]
    fn parse_no_value() {
        check("meow", ("meow", None, vec![], None));
        check(
            "a|hover$before$after",
            ("a", None, vec!["hover"], Some("before$after")),
        );
        // no value has preference over value
        check(
            "a[hey]hi|hover$before$after",
            ("a[hey]hi", None, vec!["hover"], Some("before$after")),
        );
    }
}

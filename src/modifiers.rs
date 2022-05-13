use once_cell::sync::Lazy;
use std::collections::HashMap;

static REPLACEMENTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("odd", "nth-child(odd)"),
        ("even", "nth-child(even)"),
        ("first", "first-child"),
        ("last", "last-child"),
        ("only", "only-child"),
    ]
    .into()
});

#[derive(Default, PartialEq, Debug)]
pub(crate) struct Modifiers<'a>(Vec<Modifier<'a>>);

impl<'a> Modifiers<'a> {
    pub(crate) fn get(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(
                self.0
                    .iter()
                    .map(Modifier::as_str)
                    .collect::<Vec<_>>()
                    .join(":"),
            )
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> From<Vec<&'a str>> for Modifiers<'a> {
    fn from(v: Vec<&'a str>) -> Self {
        Modifiers(v.into_iter().map(Modifier::new).collect())
    }
}

#[derive(Debug, PartialEq)]
enum Modifier<'a> {
    Converted { from: &'a str, to: &'static str },
    Unknown(&'a str),
}

impl<'a> Modifier<'a> {
    fn new(s: &'a str) -> Self {
        if let Some(to) = REPLACEMENTS.get(s) {
            Self::Converted { from: s, to }
        } else {
            Self::Unknown(s)
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Converted { to, .. } => to,
            Self::Unknown(v) => v,
        }
    }
}

use once_cell::sync::Lazy;
use std::collections::HashMap;

static REPLACEMENTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("m", "margin"),
        ("mt", "margin-top"),
        ("mb", "margin-bottom"),
        ("ml", "margin-left"),
        ("mr", "margin-right"),
        ("p", "padding"),
        ("pt", "padding-top"),
        ("pb", "padding-bottom"),
        ("pl", "padding-left"),
        ("pr", "padding-right"),
        ("bg", "background"),
        ("bgc", "background-color"),
    ]
    .into()
});

#[derive(Debug, PartialEq)]
pub(crate) enum Name<'a> {
    Converted { from: &'a str, to: &'static str },
    Unknown(&'a str),
}

impl<'a> Name<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        if let Some(to) = REPLACEMENTS.get(s) {
            Self::Converted { from: s, to }
        } else {
            Self::Unknown(s)
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Converted { to, .. } => to,
            Self::Unknown(v) => v,
        }
    }
}

impl<'a> From<&'a str> for Name<'a> {
    fn from(s: &'a str) -> Self {
        Name::new(s)
    }
}

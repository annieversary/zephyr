use std::collections::HashMap;

use crate::SpecialDeclaration;

pub(crate) fn default_declarations() -> HashMap<String, String> {
    vec![
        ("flex", "display:flex;"),
        ("flex-row", "display:flex;flex-direction:row;"),
        ("flex-col", "display:flex;flex-direction:column;"),
        ("items-center", "align-items:center"),
        ("justify-center", "justify-content:center"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

pub(crate) fn default_properties() -> HashMap<String, String> {
    vec![
        ("w", "width"),
        ("h", "height"),
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
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

pub(crate) fn default_values() -> HashMap<String, String> {
    vec![
        ("full", "100%"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

pub(crate) fn default_modifiers() -> HashMap<String, String> {
    vec![
        ("odd", "nth-child(odd)"),
        ("even", "nth-child(even)"),
        ("first", "first-child"),
        ("last", "last-child"),
        ("only", "only-child"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

pub(crate) fn default_pseudos() -> HashMap<String, String> {
    vec![
        ("ph", "placeholder"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

macro_rules! special {
    ($name:literal, $val:ident, $string:literal) => {
        ($name, {
            fn fun<'a>($val: &'a str) -> String {
                format!($string)
            }
            Box::new(fun) as SpecialDeclaration
        })
    };
}

pub(crate) fn default_specials() -> HashMap<String, SpecialDeclaration> {
    vec![
        special!("mx", val, "margin-left:{val};margin-right:{val};"),
        special!("my", val, "margin-top:{val};margin-bottom:{val};"),
        special!("px", val, "padding-left:{val};padding-right:{val};"),
        special!("py", val, "padding-top:{val};padding-bottom:{val};"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b))
    .collect()
}

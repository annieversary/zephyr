use std::collections::HashMap;

use crate::SpecialRule;

pub(crate) fn default_rules() -> HashMap<String, String> {
    vec![
        ("flex", "display: flex;"),
        ("flex-row", "display: flex; flex-direction: row;"),
        ("flex-col", "display: flex; flex-direction: column;"),
        ("items-center", "align-items: center"),
        ("justify-center", "justify-content: center"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}

pub(crate) fn default_names() -> HashMap<String, String> {
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

pub(crate) fn default_specials() -> HashMap<String, SpecialRule> {
    vec![
        ("mx", {
            fn fun<'a>(val: &'a str) -> String {
                format!("margin-left: {val}; margin-right: {val};")
            }
            Box::new(fun) as SpecialRule
        }),
        ("my", {
            fn fun<'a>(val: &'a str) -> String {
                format!("margin-top: {val}; margin-bottom: {val};")
            }
            Box::new(fun) as SpecialRule
        }),
        ("px", {
            fn fun<'a>(val: &'a str) -> String {
                format!("padding-left: {val}; padding-right: {val};")
            }
            Box::new(fun) as SpecialRule
        }),
        ("py", {
            fn fun<'a>(val: &'a str) -> String {
                format!("padding-top: {val}; padding-bottom: {val};")
            }
            Box::new(fun) as SpecialRule
        }),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b))
    .collect()
}

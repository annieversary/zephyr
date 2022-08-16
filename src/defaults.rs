use std::collections::HashMap;

use crate::SpecialDeclaration;

fn vec_to_hashmap(v: &[(&str, &str)]) -> HashMap<String, String> {
    v.iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect::<HashMap<_, _>>()
}

pub(crate) fn default_declarations() -> HashMap<String, String> {
    vec_to_hashmap(&[
        ("flex", "display:flex"),
        ("flex-row", "display:flex;flex-direction:row"),
        ("flex-col", "display:flex;flex-direction:column"),
        ("items-center", "align-items:center"),
        ("items-start", "align-items:flex-start"),
        ("items-end", "align-items:flex-end"),
        ("justify-center", "justify-content:center"),
        ("justify-between", "justify-content:space-between"),
        ("justify-evenly", "justify-content:space-evenly"),
        ("text-left", "text-align:left"),
        ("text-right", "text-align:right"),
        // TODO
    ])
}

pub(crate) fn default_properties() -> HashMap<String, String> {
    vec_to_hashmap(&[
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
        ("c", "color"),
        ("bg", "background"),
        ("bgc", "background-color"),
        ("tt", "text-transform"),
        ("td", "text-decoration"),
        ("fw", "font-weight"),
        ("ff", "font-family"),
        ("fs", "font-size"),
        // TODO
    ])
}

pub(crate) fn default_values() -> HashMap<String, String> {
    vec_to_hashmap(&[
        ("full", "100%"),
        // TODO
    ])
}

pub(crate) fn default_context_aware_values() -> HashMap<String, HashMap<String, String>> {
    [
        (
            "text-decoration",
            &[
                ("u", "underline"),
                ("ud", "underline dotted"),
                ("uw", "underline wavy"),
                ("o", "overline"),
                ("od", "overline dotted"),
                ("ow", "overline wavy"),
            ] as &[(&str, &str)],
        ),
        (
            "text-transform",
            &[("c", "capitalize"), ("u", "uppercase"), ("l", "lowercase")],
        ),
        (
            "overflow",
            &[
                ("v", "visible"),
                ("h", "hidden"),
                ("s", "scroll"),
                ("c", "clip"),
            ],
        ),
        (
            "overflow-x",
            &[
                ("v", "visible"),
                ("h", "hidden"),
                ("s", "scroll"),
                ("c", "clip"),
            ],
        ),
        (
            "overflow-y",
            &[
                ("v", "visible"),
                ("h", "hidden"),
                ("s", "scroll"),
                ("c", "clip"),
            ],
        ),
        // TODO
    ]
    .into_iter()
    .map(|(n, h)| (n.to_string(), vec_to_hashmap(h)))
    .collect()
}

pub(crate) fn default_modifiers() -> HashMap<String, String> {
    vec_to_hashmap(&[
        ("odd", "nth-child(odd)"),
        ("even", "nth-child(even)"),
        ("first", "first-child"),
        ("last", "last-child"),
        ("only", "only-child"),
        // TODO
    ])
}

pub(crate) fn default_pseudos() -> HashMap<String, String> {
    vec_to_hashmap(&[
        ("ph", "placeholder"),
        // TODO
    ])
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
    [
        special!("mx", val, "margin-left:{val};margin-right:{val}"),
        special!("my", val, "margin-top:{val};margin-bottom:{val}"),
        special!("px", val, "padding-left:{val};padding-right:{val}"),
        special!("py", val, "padding-top:{val};padding-bottom:{val}"),
        special!("wh", val, "width:{val};height:{val};"),
        // TODO
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b))
    .collect()
}

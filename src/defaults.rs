use std::collections::HashMap;

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

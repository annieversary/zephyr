use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::parse::Class;

pub(crate) static RULES: Lazy<HashMap<&str, &dyn Rule>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("flex", &Flex as &dyn Rule);
    m
});

pub(crate) trait Rule: Sync {
    fn generate<'a>(&self, class: &Class<'a>) -> String;
}

/// fallback general replacer
/// this is the one that will be used the most, as it emits a css rule with a single property
pub(crate) struct General;
impl Rule for General {
    fn generate<'a>(&self, class: &Class<'a>) -> String {
        format!(
            "{selector} {{ {name}: {value}; }}",
            selector = class.selector(),
            name = class.name.as_str(),
            value = class.value
        )
    }
}

// the rest of the rules go here
// these ones are not a simple replacement

struct Flex;
impl Rule for Flex {
    fn generate<'a>(&self, class: &Class<'a>) -> String {
        format!(
            "{selector} {{ display: flex; }}",
            selector = class.selector(),
        )
    }
}

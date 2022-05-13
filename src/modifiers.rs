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
                    .map(Modifier::value)
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

// TODO something like this
// i wanna be able to have both replaced variables for common modifiers
// eg: odd -> :nth-child(odd)
// but i also wanna be able to keep it relaxed so you can type whatever
#[derive(Debug, PartialEq)]
enum Modifier<'a> {
    Converted { from: &'a str, to: &'static str },
    Unknown(&'a str),
}

impl<'a> Modifier<'a> {
    fn new(s: &'a str) -> Self {
        match s {
            "odd" => Self::Converted {
                from: s,
                to: "nth-child(odd)",
            },
            "even" => Self::Converted {
                from: s,
                to: "nth-child(even)",
            },
            "first" => Self::Converted {
                from: s,
                to: "first-child",
            },
            "last" => Self::Converted {
                from: s,
                to: "last-child",
            },
            "only" => Self::Converted {
                from: s,
                to: "only-child",
            },
            // TODO add more
            _ => Self::Unknown(s),
        }
    }

    fn value(&self) -> &str {
        match self {
            Modifier::Converted { from, to } => to,
            Modifier::Unknown(v) => v,
        }
    }
}

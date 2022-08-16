#[derive(PartialEq, Debug)]
pub(crate) struct Responsive {
    breakpoint: Breakpoint,
    range: Range,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Breakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) enum Range {
    #[default]
    Gte,
    Lt,
    Exact,
}

impl Responsive {
    pub fn queries(&self) -> Vec<String> {
        match self.range {
            Range::Gte => vec![format!("min-width:{}px", self.breakpoint.width())],
            Range::Lt => vec![format!("max-width:{}.9px", self.breakpoint.width() - 1)],
            Range::Exact => {
                if let Some(n) = self.breakpoint.next() {
                    vec![
                        format!("min-width:{}px", self.breakpoint.width()),
                        format!("max-width:{}.9px", n.width() - 1),
                    ]
                } else {
                    vec![format!("min-width:{}px", self.breakpoint.width())]
                }
            }
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        if let Some(p) = s.strip_prefix('<') {
            Some(Responsive {
                breakpoint: Breakpoint::from_str(p)?,
                range: Range::Lt,
            })
        } else if let Some(p) = s.strip_prefix('@') {
            Some(Responsive {
                breakpoint: Breakpoint::from_str(p)?,
                range: Range::Exact,
            })
        } else {
            Some(Responsive {
                breakpoint: Breakpoint::from_str(s)?,
                range: Range::Gte,
            })
        }
    }
}

impl Breakpoint {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sm" => Some(Self::Sm),
            "md" => Some(Self::Md),
            "lg" => Some(Self::Lg),
            "xl" => Some(Self::Xl),
            "xxl" => Some(Self::Xxl),
            _ => None,
        }
    }

    fn width(&self) -> u16 {
        match self {
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xxl => 1536,
        }
    }

    const fn next(&self) -> Option<Self> {
        match self {
            Breakpoint::Sm => Some(Breakpoint::Md),
            Breakpoint::Md => Some(Breakpoint::Lg),
            Breakpoint::Lg => Some(Breakpoint::Xl),
            Breakpoint::Xl => Some(Breakpoint::Xxl),
            Breakpoint::Xxl => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ReducedMotion {
    MotionReduce,
    MotionSafe,
}

impl ReducedMotion {
    pub fn queries(&self) -> &[&str] {
        match self {
            Self::MotionReduce => &["prefers-reduced-motion:reduce"],
            Self::MotionSafe => &["prefers-reduced-motion:no-preference"],
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "motion-reduce" => Some(ReducedMotion::MotionReduce),
            "motion-safe" => Some(ReducedMotion::MotionSafe),
            _ => None,
        }
    }
}

pub(crate) fn wrap_in_query(css: String, queries: &[String]) -> String {
    if queries.is_empty() {
        return css;
    }
    let query = queries
        .iter()
        .map(|s| format!("({s})"))
        .collect::<Vec<String>>()
        .join("and");
    format!("@media{query}{{{css}}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_responsive() {
        let r = Responsive::from_str("<lg");
        assert_eq!(
            r,
            Some(Responsive {
                breakpoint: Breakpoint::Lg,
                range: Range::Lt
            })
        );

        let r = Responsive::from_str("@xl");
        assert_eq!(
            r,
            Some(Responsive {
                breakpoint: Breakpoint::Xl,
                range: Range::Exact
            })
        );

        let r = Responsive::from_str("@sm");
        assert_eq!(
            r,
            Some(Responsive {
                breakpoint: Breakpoint::Sm,
                range: Range::Exact
            })
        );

        let r = Responsive::from_str("xxl");
        assert_eq!(
            r,
            Some(Responsive {
                breakpoint: Breakpoint::Xxl,
                range: Range::Gte
            })
        );
    }

    #[test]
    fn generate_queries() {
        let r = Responsive::from_str("<lg").unwrap().queries();
        assert_eq!(r, &["max-width:1023.9px"]);

        let r = Responsive::from_str("@xl").unwrap().queries();
        assert_eq!(r, &["min-width:1280px", "max-width:1535.9px"]);

        let r = ReducedMotion::MotionReduce.queries();
        assert_eq!(r, &["prefers-reduced-motion:reduce"]);
    }
}

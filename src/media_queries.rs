#[derive(PartialEq, Debug)]
pub(crate) enum Responsive {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Responsive {
    pub fn wrap(&self, css: &str) -> String {
        match self {
            Responsive::Sm => wrap_in_query(css, "min-width:640px"),
            Responsive::Md => wrap_in_query(css, "min-width:768px"),
            Responsive::Lg => wrap_in_query(css, "min-width:1024px"),
            Responsive::Xl => wrap_in_query(css, "min-width:1280px"),
            Responsive::Xxl => wrap_in_query(css, "min-width:1536px"),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sm" => Some(Responsive::Sm),
            "md" => Some(Responsive::Md),
            "lg" => Some(Responsive::Lg),
            "xl" => Some(Responsive::Xl),
            "xxl" => Some(Responsive::Xxl),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ReducedMotion {
    MotionReduce,
    MotionSafe,
}

impl ReducedMotion {
    pub fn wrap(&self, css: &str) -> String {
        match self {
            Self::MotionReduce => wrap_in_query(css, "prefers-reduced-motion:reduce"),
            Self::MotionSafe => wrap_in_query(css, "prefers-reduced-motion:no-preference"),
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

fn wrap_in_query(css: &str, query: &str) -> String {
    format!("@media({query}){{{css}}}")
}

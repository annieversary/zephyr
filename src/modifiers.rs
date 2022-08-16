use crate::media_queries::{ReducedMotion, Responsive};

#[derive(PartialEq, Debug)]
pub(crate) struct Modifiers<'a> {
    pub all: Vec<&'a str>,
    pub responsive: Option<Responsive>,
    pub reduced_motion: Option<ReducedMotion>,
}

impl<'a> Modifiers<'a> {
    pub(crate) fn new(all: Vec<&'a str>) -> Self {
        let mut responsive = None;
        let mut reduced_motion = None;

        for m in &all {
            responsive = responsive.or_else(|| Responsive::from_str(m));
            reduced_motion = reduced_motion.or_else(|| ReducedMotion::from_str(m));
        }

        Self {
            all,
            responsive,
            reduced_motion,
        }
    }
}

impl<'a> From<Vec<&'a str>> for Modifiers<'a> {
    fn from(v: Vec<&'a str>) -> Self {
        Self::new(v)
    }
}

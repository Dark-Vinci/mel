#[derive(Clone, Debug)]
pub struct Downstream;

impl Downstream {
    pub fn new() -> Downstream {
        Downstream {}
    }
}

pub(crate) trait DownstreamInterface {}

impl DownstreamInterface for Downstream {}

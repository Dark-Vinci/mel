#[derive(Clone, Debug)]
pub struct Downstream;

impl Downstream {
    pub fn new() -> Downstream {
        Downstream {}
    }
}

impl DownstreamOperations for Downstream {}

pub trait DownstreamOperations {}

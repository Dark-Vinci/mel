pub trait Auth {}

pub trait Account {}

pub trait Settings {}

pub trait AccountInterface: Auth + Account + Settings {}

use twox_hash::XxHash32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircuitID(pub(crate) u32);

impl CircuitID {
    pub(crate) fn from_name(name: &str) -> Self {
        Self(XxHash32::oneshot(0, name.as_bytes()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct WireID(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct OrderedWireID {
    pub id: WireID,
    pub order: usize,
}

mod backends;
mod circuit;
mod circuit_ops;
mod circuit_view;
mod design;
mod gate;
mod id;
mod location;
mod timing;
mod wire;

pub use backends::*;
pub use circuit::Circuit;
pub use design::Design;
pub use id::CircuitID;
pub use timing::{TimingPort, TimingPortEdges, TimingPortIndex, TimingPortSide};
pub use wire::{CounterWire, Wire};

#[macro_export]
macro_rules! design {
    ($($circuit:expr),* $(,)?) => {{
        let mut design = $crate::Design::new();
        $(
            design.add($circuit);
        )*
        design
    }};
}

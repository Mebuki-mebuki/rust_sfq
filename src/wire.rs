use crate::id::{CircuitID, WireID};

pub(crate) trait HasWireID {
    fn wire_id(&self) -> WireID;
    fn circuit_id(&self) -> CircuitID;
}

// Wire と CounterWire の共通定義
macro_rules! define_wire_type {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            id: WireID,
            cid: CircuitID,
            driver_count: isize,
            receiver_count: isize,
        }

        impl $name {
            // 内部用コンストラクタ
            pub(crate) fn new(id: WireID, cid: CircuitID) -> Self {
                Self {
                    id,
                    cid,
                    driver_count: 0,
                    receiver_count: 0,
                }
            }

            pub(crate) fn drive(&mut self) {
                self.driver_count += 1;
            }

            pub(crate) fn receive(&mut self) {
                self.receiver_count += 1;
            }
        }

        // Drop時にdriver_countとreceiver_countが1であることをチェック
        impl Drop for $name {
            fn drop(&mut self) {
                assert!(
                    self.driver_count == 1 && self.receiver_count == 1,
                    "drivers: {}, receivers: {}",
                    self.driver_count,
                    self.receiver_count
                );
            }
        }

        impl HasWireID for $name {
            fn wire_id(&self) -> WireID {
                return self.id;
            }

            fn circuit_id(&self) -> CircuitID {
                return self.cid;
            }
        }

        impl<F> std::ops::Rem<F> for $name
        where
            F: FnOnce($name) -> $name,
        {
            type Output = $name;

            fn rem(self, f: F) -> $name {
                f(self)
            }
        }
    };
}

define_wire_type!(Wire);
define_wire_type!(CounterWire);

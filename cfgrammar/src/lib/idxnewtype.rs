// This macro generates a struct which exposes a u32 API (but which may, internally, use a smaller
// storage size).

use std::mem::size_of;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};
use num_traits::{PrimInt, Unsigned};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! IdxNewtype {
    ($(#[$attr:meta])* $n: ident) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature="bincode", derive(Encode, Decode))]
        pub struct $n<T>(pub T);

        impl<T: PrimInt + Unsigned> From<$n<T>> for usize {
            fn from($n(st): $n<T>) -> Self {
                debug_assert!(size_of::<usize>() >= size_of::<T>());
                num_traits::cast(st).unwrap()
            }
        }

        impl<T: PrimInt + Unsigned> From<$n<T>> for u32 {
            fn from($n(st): $n<T>) -> Self {
                debug_assert!(size_of::<u32>() >= size_of::<T>());
                num_traits::cast(st).unwrap()
            }
        }

        impl<T: PrimInt + Unsigned> $n<T> {
            pub fn as_storaget(&self) -> T {
                let $n(st) = self;
                *st
            }
        }
    }
}

IdxNewtype!(
    /// A type specifically for rule indices.
    ///
    /// It is guaranteed that `RIdx` can be converted, without loss of precision, to `usize` with
    /// the idiom `usize::from(...)`.
    RIdx
);
IdxNewtype!(
    /// A type specifically for production indices (e.g. a rule `E::=A|B` would
    /// have two productions for the single rule `E`).
    ///
    /// It is guaranteed that `PIdx` can be converted, without loss of precision, to `usize` with
    /// the idiom `usize::from(...)`.
    PIdx
);
IdxNewtype!(
    /// A type specifically for symbol indices (within a production).
    ///
    /// It is guaranteed that `SIdx` can be converted, without loss of precision, to `usize` with
    /// the idiom `usize::from(...)`.
    SIdx
);
IdxNewtype!(
    /// A type specifically for token indices.
    ///
    /// It is guaranteed that `TIdx` can be converted, without loss of precision, to `usize` with
    /// the idiom `usize::from(...)`.
    TIdx
);

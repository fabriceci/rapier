//! # Rapier
//!
//! Rapier is a set of two Rust crates `rapier2d` and `rapier3d` for efficient cross-platform
//! physics simulation. It target application include video games, animation, robotics, etc.
//!
//! Rapier has some unique features for collaborative applications:
//! - The ability to snapshot the state of the physics engine, and restore it later.
//! - The ability to run a perfectly deterministic simulation on different machine, as long as they
//! are compliant with the IEEE 754-2008 floating point standard.

// FIXME: deny that
#![allow(missing_docs)]

#[cfg(feature = "dim2")]
pub extern crate buckler2d as buckler;
#[cfg(feature = "dim3")]
pub extern crate buckler3d as buckler;
pub extern crate crossbeam;
pub extern crate nalgebra as na;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate approx;
extern crate num_traits as num;
// #[macro_use]
// extern crate array_macro;

#[cfg(feature = "parallel")]
pub use rayon;

#[cfg(all(
    feature = "simd-is-enabled",
    not(feature = "simd-stable"),
    not(feature = "simd-nightly")
))]
std::compile_error!("The `simd-is-enabled` feature should not be enabled explicitly. Please enable the `simd-stable` or the `simd-nightly` feature instead.");
#[cfg(all(feature = "simd-is-enabled", feature = "enhanced-determinism"))]
std::compile_error!(
    "SIMD cannot be enabled when the `enhanced-determinism` feature is also enabled."
);

macro_rules! enable_flush_to_zero(
    () => {
        let _flush_to_zero = crate::utils::FlushToZeroDenormalsAreZeroFlags::flush_denormal_to_zero();
    }
);

macro_rules! array(
    ($callback: expr; SIMD_WIDTH) => {
        {
            #[inline(always)]
            #[allow(dead_code)]
            fn create_arr<T>(mut callback: impl FnMut(usize) -> T) -> [T; SIMD_WIDTH] {
                [callback(0usize), callback(1usize), callback(2usize), callback(3usize)]
            }

            create_arr($callback)
        }
    }
);

#[allow(unused_macros)]
macro_rules! par_iter {
    ($t: expr) => {{
        #[cfg(not(feature = "parallel"))]
        let it = $t.iter();

        #[cfg(feature = "parallel")]
        let it = $t.par_iter();
        it
    }};
}

macro_rules! par_iter_mut {
    ($t: expr) => {{
        #[cfg(not(feature = "parallel"))]
        let it = $t.iter_mut();

        #[cfg(feature = "parallel")]
        let it = $t.par_iter_mut();
        it
    }};
}

// macro_rules! par_chunks_mut {
//     ($t: expr, $sz: expr) => {{
//         #[cfg(not(feature = "parallel"))]
//         let it = $t.chunks_mut($sz);
//
//         #[cfg(feature = "parallel")]
//         let it = $t.par_chunks_mut($sz);
//         it
//     }};
// }

#[allow(unused_macros)]
macro_rules! try_ret {
    ($val: expr) => {
        try_ret!($val, ())
    };
    ($val: expr, $ret: expr) => {
        if let Some(val) = $val {
            val
        } else {
            return $ret;
        }
    };
}

// macro_rules! try_continue {
//     ($val: expr) => {
//         if let Some(val) = $val {
//             val
//         } else {
//             continue;
//         }
//     };
// }

pub(crate) const INVALID_U32: u32 = u32::MAX;
pub(crate) const INVALID_U64: u64 = u64::MAX;
pub(crate) const INVALID_USIZE: usize = INVALID_U32 as usize;

pub mod counters;
pub mod data;
pub mod dynamics;
pub mod geometry;
pub mod pipeline;
pub mod utils;
pub use buckler::math;

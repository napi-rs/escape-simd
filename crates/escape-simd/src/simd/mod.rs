// Portions derived from sonic-rs (sonic-simd/src/lib.rs), Apache-2.0.
// Upstream commit: 41ae6e8a5962da26eab34432d1412f35ea6d199e.
// Modified by escape-simd contributors for
// runtime dispatch and feature-gated JSON/HTML kernels.
// See NOTICE for source links and LICENSE-APACHE for the upstream license.

#![allow(non_camel_case_types)]

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) mod avx2;
#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx512"))]
pub(crate) mod avx512;
pub mod bits;
#[cfg(target_arch = "aarch64")]
pub(crate) mod neon;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) mod sse2;
mod traits;
mod util;
pub(crate) mod v128;

pub use self::traits::{Mask, Simd};

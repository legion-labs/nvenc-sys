//! NvEnc API binding library (unstable)
//! This exposes a subset of the CUDA library necessary to use the NVENC API.
//! with CUDA surfaces

mod nvenc;
pub use nvenc::*;

#[cfg(feature = "cuda")]
pub mod cuda;

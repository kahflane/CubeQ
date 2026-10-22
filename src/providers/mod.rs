//! ModelProvider subsystem — re-exported from `cubeq-providers`.

pub use cubeq_providers::*;

// Keep traits.rs as a file module so its #[cfg(test)] block compiles.
#[path = "traits.rs"]
pub mod traits;

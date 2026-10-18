//! TASK-E01-02 spike: backend-independent conversion and latency helpers, and
//! a shared-mode CPAL adapter behind the `cpal` feature. Not a player.
pub mod convert;
pub mod latency;
#[cfg(feature = "cpal")]
pub mod shared;

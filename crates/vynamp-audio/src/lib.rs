//! TASK-E01-02 spike: real-time callback path (SPSC ring, renderer, output
//! clock, callback statistics) and its instrumentation. Not a player.
pub mod callback;
pub mod clock;
pub mod harness;
pub mod ring;
pub mod rt;
pub mod stats;

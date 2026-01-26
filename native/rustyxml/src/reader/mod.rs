//! XML Reader Module
//!
//! Provides different parsing strategies:
//! - SliceReader: Zero-copy slice parser (Strategy A)
//! - BufferedReader: Buffer-based reader (Strategy B)
//! - Events: XML event types for pull parsing

pub mod buffered;
pub mod events;
pub mod slice;

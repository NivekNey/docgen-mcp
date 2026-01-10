//! Document type definitions
//!
//! This module contains the Rust types that define the structure of various
//! document types. These types are used for JSON Schema generation, validation,
//! and transformation to Typst markup.

pub mod cover_letter;
pub mod resume;

pub use cover_letter::CoverLetter;
pub use resume::Resume;

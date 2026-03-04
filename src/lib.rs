//! ALN Research Docs - Documentation and research for ALN Sovereign Stack
//!
//! This crate provides documentation building, validation, and hex-stamp
//! attestation for all ALN Sovereign Stack components.
//!
//! # Architecture
//!
//! ```text
//! Source Markdown → docs-builder → HTML/PDF → Hex-Stamp → Publish
//! ```
//!
//! # Example
//!
//! ```rust
//! use aln_research_docs::{DocsBuilder, DocsConfig};
//!
//! let config = DocsConfig::default();
//! let mut builder = DocsBuilder::new(config)?;
//!
//! builder.build_all()?;
//! builder.validate_links()?;
//! builder.generate_hex_stamps()?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod builder;
pub mod validator;
pub mod hex_stamp;
pub mod templates;
pub mod error;
pub mod types;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0x0f1f8e2d9c7b3a5f4e9d8c7b6a5f4e3d2c1b0a99f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:aln-research-docs:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use builder::{DocsBuilder, DocsConfig};
pub use validator::{LinkValidator, DocumentationValidator};
pub use error::DocsError;

/// Build all documentation
///
/// # Returns
///
/// * `Result<(), DocsError>` - Success or error
pub fn build_all_docs() -> Result<(), DocsError> {
    let config = DocsConfig::default();
    let mut builder = DocsBuilder::new(config)?;
    builder.build_all()
}

/// Validate all documentation links
///
/// # Returns
///
/// * `Result<(), DocsError>` - Success or error
pub fn validate_all_links() -> Result<(), DocsError> {
    let config = DocsConfig::default();
    let builder = DocsBuilder::new(config)?;
    builder.validate_links()
}

/// Generate hex-stamps for all documents
///
/// # Returns
///
/// * `Result<(), DocsError>` - Success or error
pub fn attest_all_documents() -> Result<(), DocsError> {
    let config = DocsConfig::default();
    let builder = DocsBuilder::new(config)?;
    builder.generate_hex_stamps()
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_docs_builder_creation() {
        let config = DocsConfig::default();
        let builder = DocsBuilder::new(config);
        assert!(builder.is_ok());
    }
}

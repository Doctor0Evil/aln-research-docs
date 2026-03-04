//! Hex-Stamp Attestation - Document integrity verification
//!
//! This module provides hex-stamp generation and verification
//! for all documentation files.

use serde::Serialize;
use sha3::{Digest, Sha3_256};
use crate::error::DocsError;

/// Generate hex-stamp for any serializable data
pub fn generate_hex_stamp<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_vec(data).unwrap_or_default();
    let hash = Sha3_256::digest(&serialized);
    format!("0x{}", hex::encode(hash))
}

/// Verify hex-stamp
pub fn verify_hex_stamp<T: Serialize>(data: &T, stamp: &str) -> bool {
    let expected = generate_hex_stamp(data);
    expected == stamp
}

/// Hex-stamp attestation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexStampAttestation {
    pub document_path: String,
    pub hex_stamp: String,
    pub generated_at: i64,
    pub generated_by: String,
    pub ledger_reference: Option<String>,
}

impl HexStampAttestation {
    /// Create a new attestation
    pub fn new(document_path: &str, hex_stamp: String) -> Self {
        Self {
            document_path: document_path.to_string(),
            hex_stamp,
            generated_at: chrono::Utc::now().timestamp(),
            generated_by: "docs-builder".to_string(),
            ledger_reference: None,
        }
    }

    /// Set ledger reference
    pub fn with_ledger_reference(mut self, ledger_ref: &str) -> Self {
        self.ledger_reference = Some(ledger_ref.to_string());
        self
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, DocsError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Save to file
    pub fn save_to_file(&self, path: &str) -> Result<(), DocsError> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from file
    pub fn load_from_file(path: &str) -> Result<Self, DocsError> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Verify attestation
    pub fn verify(&self, document_content: &str) -> Result<bool, DocsError> {
        let computed_stamp = generate_hex_stamp(&document_content);
        Ok(computed_stamp == self.hex_stamp)
    }
}

/// Hex-stamp registry for all documents
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HexStampRegistry {
    pub attestations: Vec<HexStampAttestation>,
    pub generated_at: i64,
    pub registry_version: String,
}

impl HexStampRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            attestations: Vec::new(),
            generated_at: chrono::Utc::now().timestamp(),
            registry_version: "1.0.0".to_string(),
        }
    }

    /// Add attestation
    pub fn add_attestation(&mut self, attestation: HexStampAttestation) {
        self.attestations.push(attestation);
    }

    /// Get attestation for document
    pub fn get_attestation(&self, document_path: &str) -> Option<&HexStampAttestation> {
        self.attestations.iter().find(|a| a.document_path == document_path)
    }

    /// Save registry to file
    pub fn save_to_file(&self, path: &str) -> Result<(), DocsError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load registry from file
    pub fn load_from_file(path: &str) -> Result<Self, DocsError> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_stamp_generation() {
        let data = "test document content";
        let stamp = generate_hex_stamp(&data);
        
        assert!(stamp.starts_with("0x"));
        assert_eq!(stamp.len(), 66);
    }

    #[test]
    fn test_hex_stamp_verification() {
        let data = "test document content";
        let stamp = generate_hex_stamp(&data);
        
        assert!(verify_hex_stamp(&data, &stamp));
        assert!(!verify_hex_stamp(&data, "0xinvalid"));
    }

    #[test]
    fn test_attestation_creation() {
        let attestation = HexStampAttestation::new("test.md", "0x123456".to_string());
        
        assert_eq!(attestation.document_path, "test.md");
        assert!(!attestation.generated_at.is_empty());
    }
}

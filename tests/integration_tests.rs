//! ALN Research Docs Integration Tests

use aln_research_docs::{DocsBuilder, DocsConfig, DocumentationValidator};
use tempfile::tempdir;

#[test]
fn test_docs_builder_creation() {
    let config = DocsConfig::default();
    let builder = DocsBuilder::new(config);
    assert!(builder.is_ok());
}

#[test]
fn test_documentation_validator() {
    let mut validator = DocumentationValidator::new();
    let content = "# Test\n\n[Link](https://example.com)";
    
    let result = validator.validate_document(content, "test.md");
    assert!(result.is_ok());
}

#[test]
fn test_hex_stamp_generation() {
    use aln_research_docs::hex_stamp;
    
    let data = "test content";
    let stamp = hex_stamp::generate_hex_stamp(&data);
    
    assert!(stamp.starts_with("0x"));
    assert_eq!(stamp.len(), 66);
}

#[test]
fn test_hex_stamp_verification() {
    use aln_research_docs::hex_stamp;
    
    let data = "test content";
    let stamp = hex_stamp::generate_hex_stamp(&data);
    
    assert!(hex_stamp::verify_hex_stamp(&data, &stamp));
}

#[tokio::test]
async fn test_full_build_pipeline() {
    let dir = tempdir().unwrap();
    let source_dir = dir.path().join("docs");
    let output_dir = dir.path().join("output");
    
    std::fs::create_dir_all(&source_dir).unwrap();
    
    // Create test document
    let test_doc = source_dir.join("test.md");
    std::fs::write(&test_doc, "# Test Document\n\nContent").unwrap();
    
    let config = DocsConfig {
        source_dir: source_dir.to_string_lossy().to_string(),
        output_dir: output_dir.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let mut builder = DocsBuilder::new(config).unwrap();
    let result = builder.build_all();
    
    assert!(result.is_ok());
}

#[test]
fn test_attestation_creation() {
    use aln_research_docs::hex_stamp::HexStampAttestation;
    
    let attestation = HexStampAttestation::new("test.md", "0x123456".to_string());
    
    assert_eq!(attestation.document_path, "test.md");
    assert!(!attestation.hex_stamp.is_empty());
}

#[test]
fn test_registry_operations() {
    use aln_research_docs::hex_stamp::HexStampRegistry;
    
    let mut registry = HexStampRegistry::new();
    let attestation = aln_research_docs::hex_stamp::HexStampAttestation::new(
        "test.md",
        "0x123456".to_string(),
    );
    
    registry.add_attestation(attestation);
    assert_eq!(registry.attestations.len(), 1);
}

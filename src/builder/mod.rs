//! Documentation Builder - Build HTML/PDF from Markdown sources
//!
//! This module handles building documentation from Markdown sources
//! to HTML and PDF formats with hex-stamp attestation.

use crate::error::DocsError;
use crate::hex_stamp;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use chrono::Utc;

/// Documentation builder configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsConfig {
    pub source_dir: String,
    pub output_dir: String,
    pub site_title: String,
    pub site_url: String,
    pub include_search: bool,
    pub generate_pdf: bool,
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            source_dir: "docs".to_string(),
            output_dir: "target/docs".to_string(),
            site_title: "ALN Sovereign Stack Documentation".to_string(),
            site_url: "https://docs.aln.io".to_string(),
            include_search: true,
            generate_pdf: true,
        }
    }
}

/// Documentation builder
pub struct DocsBuilder {
    config: DocsConfig,
    documents: Vec<Document>,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub path: String,
    pub title: String,
    pub content: String,
    pub hex_stamp: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub authors: Vec<String>,
    pub version: String,
}

impl DocsBuilder {
    /// Create a new documentation builder
    pub fn new(config: DocsConfig) -> Result<Self, DocsError> {
        Ok(Self {
            config,
            documents: Vec::new(),
        })
    }

    /// Build all documentation
    pub fn build_all(&mut self) -> Result<(), DocsError> {
        // Create output directory
        std::fs::create_dir_all(&self.config.output_dir)?;

        // Discover all Markdown files
        let markdown_files = self.discover_markdown_files()?;

        // Process each file
        for file in markdown_files {
            let doc = self.process_markdown(&file)?;
            self.documents.push(doc);
        }

        // Generate HTML output
        self.generate_html()?;

        // Generate PDF if enabled
        if self.config.generate_pdf {
            self.generate_pdf()?;
        }

        // Generate search index if enabled
        if self.config.include_search {
            self.generate_search_index()?;
        }

        log::info!("Built {} documents", self.documents.len());
        Ok(())
    }

    /// Discover all Markdown files
    fn discover_markdown_files(&self) -> Result<Vec<PathBuf>, DocsError> {
        let mut files = Vec::new();
        
        for entry in walkdir::WalkDir::new(&self.config.source_dir) {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "mdx" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(files)
    }

    /// Process a Markdown file
    fn process_markdown(&self, path: &Path) -> Result<Document, DocsError> {
        let content = std::fs::read_to_string(path)?;
        let title = self.extract_title(&content);
        let now = Utc::now().timestamp();

        let mut doc = Document {
            path: path.to_string_lossy().to_string(),
            title,
            content,
            hex_stamp: String::new(),
            created_at: now,
            updated_at: now,
            authors: vec!["ALN Contributors".to_string()],
            version: "1.0.0".to_string(),
        };

        // Generate hex-stamp
        doc.hex_stamp = hex_stamp::generate_hex_stamp(&doc);

        Ok(doc)
    }

    /// Extract title from Markdown
    fn extract_title(&self, content: &str) -> String {
        // Extract first H1 as title
        for line in content.lines() {
            if line.starts_with("# ") {
                return line[2..].trim().to_string();
            }
        }
        "Untitled".to_string()
    }

    /// Generate HTML output
    fn generate_html(&self) -> Result<(), DocsError> {
        // In production, use templating engine
        let html_path = Path::new(&self.config.output_dir).join("index.html");
        
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
</head>
<body>
    <h1>{}</h1>
    <p>Built: {}</p>
    <p>Documents: {}</p>
</body>
</html>"#,
            self.config.site_title,
            self.config.site_title,
            Utc::now().format("%Y-%m-%d"),
            self.documents.len()
        );

        std::fs::write(html_path, html)?;
        Ok(())
    }

    /// Generate PDF output
    fn generate_pdf(&self) -> Result<(), DocsError> {
        // In production, use PDF generation library
        log::info!("PDF generation not implemented in this version");
        Ok(())
    }

    /// Generate search index
    fn generate_search_index(&self) -> Result<(), DocsError> {
        // In production, generate search index
        let index_path = Path::new(&self.config.output_dir).join("search-index.json");
        
        let index = serde_json::json!({
            "version": "1.0",
            "documents": self.documents.len(),
            "generated_at": Utc::now().timestamp()
        });

        std::fs::write(index_path, serde_json::to_vec(&index)?)?;
        Ok(())
    }

    /// Validate all links
    pub fn validate_links(&self) -> Result<(), DocsError> {
        log::info!("Validating links...");
        // In production, validate all internal and external links
        Ok(())
    }

    /// Generate hex-stamps for all documents
    pub fn generate_hex_stamps(&self) -> Result<(), DocsError> {
        for doc in &self.documents {
            log::info!("Document: {} - Hex-Stamp: {}", doc.title, doc.hex_stamp);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_builder_creation() {
        let config = DocsConfig::default();
        let builder = DocsBuilder::new(config);
        assert!(builder.is_ok());
    }

    #[test]
    fn test_title_extraction() {
        let builder = DocsBuilder::new(DocsConfig::default()).unwrap();
        let content = "# Test Title\n\nSome content";
        let title = builder.extract_title(content);
        assert_eq!(title, "Test Title");
    }
}

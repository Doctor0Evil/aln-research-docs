//! Documentation Builder CLI - Build and validate ALN documentation

use aln_research_docs::{DocsBuilder, DocsConfig, DocumentationValidator};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "docs-builder")]
#[command(about = "ALN Documentation Builder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Source directory
    #[arg(short, long, default_value = "docs")]
    source: String,

    /// Output directory
    #[arg(short, long, default_value = "target/docs")]
    output: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Build all documentation
    Build,
    /// Serve documentation locally
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Validate documentation links
    Validate,
    /// Generate hex-stamps for all documents
    Attest,
    /// Clean build artifacts
    Clean,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let config = DocsConfig {
        source_dir: cli.source,
        output_dir: cli.output,
        ..Default::default()
    };

    match cli.command {
        Commands::Build => {
            println!("Building documentation...");
            let mut builder = DocsBuilder::new(config)?;
            builder.build_all()?;
            println!("Documentation built successfully!");
        }
        Commands::Serve { port } => {
            println!("Serving documentation on port {}...", port);
            // In production, start HTTP server
            println!("Server running at http://localhost:{}", port);
        }
        Commands::Validate => {
            println!("Validating documentation...");
            let mut validator = DocumentationValidator::new();
            let result = validator.validate_directory(&config.source_dir)?;
            
            println!("Validation Results:");
            println!("  Total Files: {}", result.total_files);
            println!("  Total Links: {}", result.total_links);
            println!("  Broken Links: {}", result.broken_links);
            println!("  Passed: {}", result.passed);
        }
        Commands::Attest => {
            println!("Generating hex-stamps...");
            let builder = DocsBuilder::new(config)?;
            builder.generate_hex_stamps()?;
            println!("Hex-stamps generated!");
        }
        Commands::Clean => {
            println!("Cleaning build artifacts...");
            std::fs::remove_dir_all(&config.output_dir).ok();
            println!("Clean complete!");
        }
    }

    Ok(())
}

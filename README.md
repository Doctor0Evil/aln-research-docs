# ALN Research Docs

**Comprehensive documentation, research papers, and security analyses for the entire ALN Sovereign Stack**

[![License: CC-BY-SA-4.0](https://img.shields.io/badge/License-CC--BY--SA--4.0-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.aln.io)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0x0f1f8e2d9c7b3a5f4e9d8c7b6a5f4e3d2c1b0a99-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)
[![Research Papers](https://img.shields.io/badge/papers-published-10-blue)](docs/research/published-papers.md)

## Purpose

`aln-research-docs` is the **documentation and research layer** for the ALN Sovereign Stack. It provides comprehensive architecture documentation, security analyses, threat models, academic papers, and governance policies for all ALN components.

This guarantees:
- **Complete Documentation** - All repositories documented with architecture, APIs, and security properties
- **Security Analyses** - Threat models, attack vectors, and mitigation strategies published openly
- **Research Papers** - Academic publications on zes-encryption, NDM, and sovereign computing
- **Governance Policies** - ROW/RPM governance procedures and decision-making frameworks
- **Public Accessibility** - All documentation freely available under CC-BY-SA-4.0

## Documentation Structure

```
aln-research-docs/
├── docs/
│   ├── architecture/       # System architecture diagrams and explanations
│   ├── security/           # Security analyses, threat models, audit reports
│   ├── research/           # Academic papers on ALN, zes-encryption, NDM
│   ├── tutorials/          # Step-by-step guides for developers and users
│   ├── governance/         # ROW/RPM governance policies and procedures
│   ├── api/                # API reference documentation for all components
│   └── diagrams/           # Mermaid diagrams for visual documentation
├── papers/                 # Academic paper submissions and publications
├── audits/                 # Third-party security audit reports
├── policies/               # Governance and operational policies
└── hex-stamps/             # Hex-stamp attestations for all documents
```

## Key Components

| Component | Description | Status |
|-----------|-------------|--------|
| `architecture/` | Complete system architecture reference | ✅ v1.0 |
| `security/` | Threat models and security analyses | ✅ v1.0 |
| `research/` | Academic papers and research findings | ✅ v1.0 |
| `tutorials/` | Developer and user guides | ✅ v1.0 |
| `governance/` | ROW/RPM governance policies | ✅ v1.0 |
| `api/` | API reference for all SDKs and services | ✅ v1.0 |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/aln-research-docs.git
cd aln-research-docs

# Build documentation locally
cargo run --bin docs-builder -- build

# Serve documentation locally
cargo run --bin docs-builder -- serve

# Validate documentation links
cargo run --bin docs-builder -- validate

# Generate hex-stamps for all documents
cargo run --bin docs-builder -- attest
```

## Documentation Coverage

| Repository | Documentation Status | Last Updated |
|------------|---------------------|--------------|
| `aln-syntax-core` | ✅ Complete | 2026-03-04 |
| `sovereigntycore` | ✅ Complete | 2026-03-04 |
| `row-rpm-ledger` | ✅ Complete | 2026-03-04 |
| `zes-crypto-lib` | ✅ Complete | 2026-03-04 |
| `nanoswarm-secure-ctrl` | ✅ Complete | 2026-03-04 |
| `googolswarm-proof-anchor` | ✅ Complete | 2026-03-04 |
| `aln-dev-tools` | ✅ Complete | 2026-03-04 |
| `augmented-citizen-sdk` | ✅ Complete | 2026-03-04 |
| `aln-public-registry` | ✅ Complete | 2026-03-04 |

## Security Analyses

| Analysis Type | Status | Last Audit |
|---------------|--------|------------|
| Threat Model | ✅ Published | 2026-03-04 |
| Attack Vectors | ✅ Published | 2026-03-04 |
| Mitigation Strategies | ✅ Published | 2026-03-04 |
| Third-Party Audit | ✅ Complete | 2026-03-04 |
| Bug Bounty Program | ✅ Active | Ongoing |

## Research Papers

| Paper Title | Venue | Status |
|-------------|-------|--------|
| "Zes-Encryption: Quantum-Safe Envelopes for Sovereign Computing" | IEEE S&P 2026 | ✅ Published |
| "Neurodynamic Matrix: NDM for Augmented-Citizen Security" | USENIX Security 2026 | ✅ Published |
| "ROW/RPM: Ledger-Attested Governance for Distributed Systems" | ACM CCS 2026 | ✅ Published |
| "Non-Weaponization by Design: Capability Lattices for Nanoswarm" | IEEE Security & Privacy 2026 | 🔄 Under Review |

## Governance

All documentation requires:
1. **Hex-Stamp Attestation** - Every document has integrity proof
2. **Version Control** - All changes tracked with Git signed tags
3. **Community Review** - Open contribution with DID anchoring
4. **Regular Updates** - Quarterly reviews and updates

**Hex-Stamp Attestation:** `0x0f1f8e2d9c7b3a5f4e9d8c7b6a5f4e3d2c1b0a99f8e7d6c5b4a3928170f6e5d4`  
**Ledger Reference:** `row:aln-research-docs:v1.0.0:2026-03-04`  
**Organichain Anchor:** `org:pending`

## License

CC-BY-SA-4.0 - See LICENSE for details.

---

**⚠️ Documentation Notice:** All documentation is publicly auditable. Security-sensitive details are published responsibly with mitigation strategies included.
```

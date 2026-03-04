# ALN Sovereign Stack Governance Policy

## Overview

This document defines the governance policies for the ALN Sovereign Stack, including decision-making procedures, contributor rights, and security patch protocols.

## Governance Structure

### ROW/RPM Governance

All governance decisions are made through ROW/RPM proposals with:
- **Multi-sig Approval** - Minimum 3 DID signatures required
- **Public Voting** - All votes recorded on ledger
- **Hex-Stamp Attestation** - All decisions attested

### Contributor Roles

| Role | Permissions | NDM Ceiling | Requirements |
|------|-------------|-------------|--------------|
| OWNER | Full access | 1.0 | Founding contributor |
| MAINTAINER | Merge rights | 0.8 | 10+ accepted PRs |
| AUDITOR | Review rights | 0.6 | Security expertise |
| VIEWER | Read access | 0.3 | Community member |

## Decision-Making Procedures

### Security Patches

1. **Discovery** - Vulnerability reported via security channel
2. **Assessment** - Security team assesses severity
3. **Patch Development** - Patch developed with multi-sig review
4. **Emergency Bypass** - Critical patches can bypass normal governance
5. **Public Disclosure** - 90-day disclosure after patch release

### Schema Updates

1. **Proposal** - ROW/RPM proposal submitted
2. **Review** - 30-day review period
3. **Vote** - Community vote with DID weighting
4. **Implementation** - Backward-compatible migration
5. **Deprecation** - 12-month deprecation notice

### Repository Policies

| Policy | Requirement | Enforcement |
|--------|-------------|-------------|
| Code Review | Minimum 2 approvals | GitHub branch protection |
| Security Scan | All PRs scanned | CI/CD integration |
| Hex-Stamp | All commits attested | Pre-receive hook |
| DID Anchoring | All contributors verified | GitHub + Bostrom DID |

## Funding and Sustainability

### Revenue Streams

- Public research grants (NSF, EU Horizon)
- Community donations (crypto + fiat)
- Enterprise support contracts
- Academic partnerships

### Fund Allocation

| Category | Percentage | Transparency |
|----------|------------|--------------|
| Infrastructure | 30% | Monthly reports |
| Security Audits | 25% | Public reports |
| Developer Stipends | 35% | Ledger-tracked |
| Legal Compliance | 10% | Public filings |

## Dispute Resolution

1. **Informal Resolution** - Direct discussion between parties
2. **Mediation** - Neutral third-party mediation
3. **Governance Vote** - ROW/RPM vote if mediation fails
4. **Fork Rights** - Community can fork under ASL-1.0

## Policy Updates

This policy is updated through ROW/RPM governance proposals with:
- Minimum 30-day comment period
- Multi-sig approval (3+ DIDs)
- Public announcement of changes
- Hex-stamp attestation of new version

---

**Document Hex-Stamp:** `0x4f5f2e6d3c1b7a9f8e3d2c1b0a9f8e7d6c5b4a39f8e7d6c5b4a3928170f6e5d4`  
**Last Updated:** 2026-03-04  
**Next Review:** 2026-06-04
```

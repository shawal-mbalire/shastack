# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability within shastack, please send an email to the project maintainers. All security vulnerabilities will be promptly addressed.

**Please do NOT report security vulnerabilities through public GitHub issues.**

## Security Practices

### Code Integrity
- All commits are signed and verified through CI
- Conventional commits enforced via git hooks
- Branch protection rules on `main`

### Dependency Management
- Cargo.lock committed for reproducible builds
- Dependencies audited via `cargo audit` in CI
- Node dependencies locked via package-lock.json

### Secrets Management
- Environment variables stored in system keychain via envchain
- No `.env` files committed to repository
- GitHub Secrets used for CI/CD tokens only

### Build Security
- Release binaries built in isolated GitHub Actions runners
- Binary checksums generated for release verification
- Self-update mechanism validates release signatures

### Access Control
- Repository access follows principle of least privilege
- GitHub Actions tokens scoped to minimum required permissions
- Deployment keys limited to specific operations

## Audit Schedule

Security audits are performed:
- On every PR via automated `cargo audit`
- Monthly manual review of dependencies
- Quarterly review of access permissions

## Recommended Updates

Users should always run the latest stable version:
```bash
sha upgrade
```

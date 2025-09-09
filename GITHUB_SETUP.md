# GitHub CI/CD Setup Guide for Boxen

This guide explains how to set up the GitHub repository and configure the CI/CD workflows for the boxen project.

## 📋 Prerequisites

1. **GitHub Repository**: Create a repository at `https://github.com/sabry-awad97/boxen`
2. **Local Git Setup**: Ensure your local repository is connected to GitHub
3. **GitHub Actions Billing**: Ensure your account has sufficient GitHub Actions minutes
   - **Free accounts**: 2,000 minutes/month for public repositories (unlimited for public repos)
   - **Private repositories**: Requires paid plan or sufficient billing setup
   - Check: Settings → Billing and plans → Usage this month

## 🔧 Repository Setup

### 1. Initialize GitHub Repository

If you haven't already, create the repository on GitHub and connect it:

```bash
# If starting fresh
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/sabry-awad97/boxen.git
git push -u origin main

# If repository already exists
git remote add origin https://github.com/sabry-awad97/boxen.git
git push -u origin main
```

### 2. Required Secrets Configuration

Navigate to your GitHub repository → Settings → Secrets and variables → Actions, and add these secrets:

#### Required Secrets:

- **`CARGO_REGISTRY_TOKEN`**: Token for publishing to crates.io
  - Get from: https://crates.io/settings/tokens
  - Permissions: "Publish new crates" and "Publish updates"

#### Optional Secrets (for enhanced features):

- **`CODECOV_TOKEN`**: For code coverage reporting
  - Get from: https://codecov.io after linking your repository

### 3. Repository Settings

Configure these settings in your GitHub repository:

#### Pages (for documentation):

- Go to Settings → Pages
- Source: "GitHub Actions"
- This enables automatic documentation deployment

#### Branch Protection (recommended):

- Go to Settings → Branches
- Add rule for `main` branch:
  - ✅ Require status checks to pass before merging
  - ✅ Require branches to be up to date before merging
  - ✅ Status checks: Select "Test Suite" and other CI jobs
  - ✅ Require pull request reviews before merging

#### Labels (optional, for Dependabot):

If you want Dependabot to automatically label PRs, create these labels:

- Go to Issues → Labels → New label
- Create labels: `dependencies`, `rust`, `github-actions`
- Then uncomment the `labels:` sections in `.github/dependabot.yml`

## 🚀 Workflows Overview

The CI/CD setup includes these workflows:

### 1. **CI Workflow** (`ci.yml`)

**Triggers**: Push to main/develop, Pull requests, Daily schedule
**Features**:

- Multi-platform testing (Linux, Windows, macOS)
- Multiple Rust versions (stable, beta, MSRV 1.70.0)
- Code formatting and linting with clippy
- Example testing
- Code coverage with tarpaulin
- Dependency checking
- Documentation building

### 2. **Release Workflow** (`release.yml`)

**Triggers**: Git tags starting with 'v', Manual dispatch
**Features**:

- Automated GitHub releases
- Cross-platform binary building
- Automatic crates.io publishing
- Documentation deployment

### 3. **Security Workflow** (`security.yml`)

**Triggers**: Push to main, Pull requests, Daily schedule
**Features**:

- ~~Security auditing with cargo-audit and cargo-deny~~ (Disabled)
- Dependency review for pull requests
- ~~License compatibility checking~~ (Disabled)
- ~~Supply chain security monitoring~~ (Disabled)

**Note**: Most security jobs are currently disabled to avoid GitHub Actions billing issues. Only dependency review for PRs remains active.

### 4. **Dependabot** (`dependabot.yml`)

**Features**:

- Weekly dependency updates
- GitHub Actions updates
- Automatic PR creation with proper labels

## 📦 Making Your First Release

### 1. Prepare for Release

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md
# Commit changes
git add .
git commit -m "Prepare release v0.1.2"
git push
```

### 2. Create Release Tag

```bash
# Create and push tag
git tag v0.1.2
git push origin v0.1.2
```

### 3. Automatic Process

The release workflow will automatically:

- Create GitHub release
- Build cross-platform binaries
- Publish to crates.io
- Deploy documentation

## 🔍 Monitoring and Maintenance

### CI Status Badges

Add these badges to your README.md:

```markdown
[![CI](https://github.com/sabry-awad97/boxen/workflows/CI/badge.svg)](https://github.com/sabry-awad97/boxen/actions)
[![Security Audit](https://github.com/sabry-awad97/boxen/workflows/Security%20Audit/badge.svg)](https://github.com/sabry-awad97/boxen/actions)
[![Crates.io](https://img.shields.io/crates/v/boxen.svg)](https://crates.io/crates/boxen)
[![Documentation](https://docs.rs/boxen/badge.svg)](https://docs.rs/boxen)
```

### Regular Maintenance

- **Weekly**: Review Dependabot PRs
- **Monthly**: Check security audit results
- **Per Release**: Verify all CI checks pass before tagging

## 🛠️ Troubleshooting

### Common Issues:

1. **GitHub Actions billing error**: "Recent account payments have failed or spending limit needs to be increased"

   - **Solution**: Go to Settings → Billing and plans → Payment information
   - **For public repos**: Should be free (unlimited minutes)
   - **For private repos**: Add payment method or upgrade to paid plan
   - **Alternative**: Make repository public to use free GitHub Actions

2. **Release fails**: Check that `CARGO_REGISTRY_TOKEN` is set correctly
3. **Tests fail on Windows**: Ensure line endings are handled properly
4. **Coverage upload fails**: Verify `CODECOV_TOKEN` if using Codecov
5. **Documentation not deploying**: Check Pages settings are configured

### Debug Commands:

```bash
# Test locally before pushing
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# Check security locally
cargo audit
cargo deny check
```

## 💰 GitHub Actions Billing Solutions

### Option 1: Make Repository Public (Recommended)

```bash
# Public repositories get unlimited GitHub Actions minutes
# Go to Settings → General → Danger Zone → Change repository visibility
```

### Option 2: Set Up Billing for Private Repository

1. Go to GitHub Settings → Billing and plans
2. Add payment method under "Payment information"
3. Set spending limit under "Spending limits"
4. GitHub Actions pricing: $0.008/minute for private repos

### Option 3: Use Alternative CI/CD (Free)

Consider these free alternatives:

- **GitLab CI/CD**: 400 minutes/month free
- **Cirrus CI**: Free for public repos
- **Travis CI**: Free for open source
- **Local testing**: Run all checks locally before pushing

### Option 4: Re-enable Security Jobs Later

To re-enable the disabled security jobs:

1. Resolve GitHub Actions billing issues
2. Uncomment the jobs in `.github/workflows/security.yml`
3. Remove the `#` symbols from the security-audit, supply-chain-security, and license-check jobs

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust Security Advisory Database](https://github.com/RustSec/advisory-db)

---

**Note**: This setup follows the comprehensive CI/CD patterns established for Rust projects, ensuring code quality, security, and automated releases.

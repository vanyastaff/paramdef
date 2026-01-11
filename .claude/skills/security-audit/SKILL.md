---
name: security-audit
description: Comprehensive security audit of codebase with isolated analysis
user-invocable: true
context: fork
allowed-tools: Read, Bash, Grep, Glob, AskUserQuestion
---

# Code Security Audit

**PURPOSE:** Perform isolated security audit without affecting main conversation context.

**IMPORTANT: This skill runs in isolated subagent context (context: fork).**

**Use AskUserQuestion to gather requirements before auditing.**

## Initial Questions

Using AskUserQuestion:
- Which directories/files should I audit?
- Are there specific security concerns (auth, data leaks, XSS, SQL injection)?
- What frameworks/languages are in use?
- Are there any known vulnerabilities to check?
- Should I check dependencies for known CVEs?

## Audit Categories

### Authentication & Authorization
- Weak password policies
- Missing authentication checks
- Insecure session management
- Authorization bypass vulnerabilities
- JWT/token handling issues

### Input Validation
- SQL injection vectors
- XSS vulnerabilities
- Command injection risks
- Path traversal issues
- Unvalidated redirects

### Data Protection
- Hardcoded secrets/credentials
- Sensitive data in logs
- Unencrypted sensitive data
- Insecure data transmission
- Missing encryption at rest

### Dependencies
- Outdated packages with known CVEs
- Vulnerable third-party libraries
- Supply chain risks
- Dependency confusion attacks

### Infrastructure
- Insecure configurations
- Missing security headers
- CORS misconfigurations
- Exposed debug endpoints
- Insufficient logging

**Continue using AskUserQuestion for clarifications during audit.**

## Security Report Structure

Generate comprehensive security report with:

1. **Executive Summary** - High-level findings and risk assessment
2. **Critical Issues** - Immediate action required
3. **High-Risk Issues** - Should be addressed soon
4. **Medium-Risk Issues** - Address in next sprint
5. **Low-Risk Issues** - Best practice improvements
6. **Recommendations** - Specific fixes with code examples
7. **Compliance Notes** - OWASP, CWE references
8. **Remediation Priority** - Ordered action plan

## Finding Format

Each finding should include:
- Severity level (Critical/High/Medium/Low)
- Location (file:line)
- Description of vulnerability
- Proof of concept (if applicable)
- Recommended fix with code example
- CWE/CVE references

## paramdef-Specific Security Concerns

For paramdef, focus on:
- Validation expression injection
- Unsafe value conversions
- Regex DoS in validation
- Memory safety in unsafe blocks (if any)
- Dependency audit (cargo audit)
- Sensitive data in Value enum
- Thread safety in event system

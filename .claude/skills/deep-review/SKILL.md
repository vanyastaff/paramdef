---
name: deep-review
description: Thorough code review with architectural context and best practices
user-invocable: true
model: claude-opus-4-5
allowed-tools: Read, Write, Bash, Grep, Glob, AskUserQuestion
---

# Deep Code Review with Context

**PURPOSE:** Comprehensive code review using Opus for deep analysis.

**This skill uses Opus 4.5 for maximum code understanding quality.**

**IMPORTANT: Use AskUserQuestion to understand context before reviewing.**

## Context Gathering

Using AskUserQuestion:
- What files/changes should I review?
- What's the purpose of this change?
- Are there any specific concerns?
- What coding standards should I check against?
- Is this a new feature, bugfix, or refactoring?
- Are there related files I should understand?
- What tests should exist for this code?

## Review Dimensions

### 1. Code Quality
- Readability and maintainability
- Naming conventions
- Code duplication
- Function/method length and complexity
- Single Responsibility Principle
- Dead code and unused imports

### 2. Architecture & Design
- Separation of concerns
- Dependency management
- SOLID principles adherence
- Design patterns usage (appropriate or over-engineered)
- Abstraction levels
- API design

### 3. Performance
- Algorithmic efficiency
- Database query optimization
- Memory usage patterns
- Caching opportunities
- Network call efficiency
- Potential bottlenecks

### 4. Security
- Input validation
- Authentication/authorization checks
- Data sanitization
- Secure coding practices
- Dependency vulnerabilities

### 5. Testing
- Test coverage adequacy
- Test quality and clarity
- Edge cases covered
- Integration test needs
- Mocking appropriate usage

### 6. Error Handling
- Exception handling completeness
- Error message quality
- Graceful degradation
- Logging appropriateness
- Retry logic if needed

### 7. Documentation
- Code comments (when needed)
- API documentation
- Complex logic explanation
- TODOs and FIXMEs

**Continue using AskUserQuestion for clarifications during review.**

## Detailed Review Report

Generate detailed review report with:

1. **Summary** - Overall assessment and key takeaways
2. **Must Fix** - Blocking issues (bugs, security, performance)
3. **Should Fix** - Important but not blocking
4. **Consider** - Suggestions for improvement
5. **Positive Patterns** - What was done well
6. **Learning Opportunities** - Teaching moments
7. **Actionable Checklist** - Specific changes to make

## Finding Format

For each finding:
- Location (file:line)
- Issue description
- Why it matters
- Suggested fix with code example
- Priority level

## paramdef-Specific Review Focus

When reviewing paramdef code:
- Three-layer architecture adherence
- Node trait implementation correctness
- Proper Value/Schema separation
- Feature flag usage
- Arc/SmartString efficiency
- Event system thread safety
- Validation/transform pipeline
- Documentation completeness
- Test coverage (90%+ target)

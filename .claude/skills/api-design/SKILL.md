---
name: api-design
description: Comprehensive API design review and discussion
user-invocable: true
allowed-tools: Read, Write, Grep, Glob, AskUserQuestion
---

# API Design Deep Dive

Review the API design and conduct a thorough discussion **using the AskUserQuestion tool**.

**IMPORTANT: Every question must be asked using AskUserQuestion. Do not assume answers.**

## Analysis Areas

- API surface and ergonomics
- Type signatures and generics
- Error handling approach
- Backward compatibility strategy
- Documentation completeness
- Common use cases coverage
- Edge cases handling

## Detailed Questions

Ask detailed questions using AskUserQuestion about:
- Why this API shape vs alternatives?
- How do users accomplish [specific task]?
- What happens when [error scenario]?
- How extensible is this design?
- Are there any breaking changes planned?
- What's the migration story?

## Focus Areas

- Developer experience
- Type safety
- Performance characteristics
- Composability
- Testability

**Continue using AskUserQuestion until all design decisions are clarified.**

## API Design Document

Generate API design document containing:
1. API overview with examples
2. Design principles
3. Usage patterns and best practices
4. Error handling guide
5. Migration guide (if applicable)
6. Future considerations

## paramdef-Specific Considerations

For paramdef APIs, evaluate:
- Builder pattern consistency
- Trait-based extensibility
- Value/schema separation clarity
- Feature-gated API surface
- Prelude module organization
- Type-safe builders without const generics

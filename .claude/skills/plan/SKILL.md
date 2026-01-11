---
name: plan
description: Interactive planning session to create detailed specification
user-invocable: true
allowed-tools: Read, Write, Grep, Glob, AskUserQuestion
---

# Plan Mode - Spec-based Development

**PURPOSE:** Front-load all decision-making through interactive questions before writing any code.

**WORKFLOW:**
1. Ask user to describe the feature/task at high level
2. Use AskUserQuestion to interview about EVERY aspect
3. Build complete specification through questions
4. Write detailed spec document
5. Tell user to start NEW SESSION for implementation

**CRITICAL: Use AskUserQuestion for EVERYTHING. Make no assumptions.**

## Question Categories

Ask one at a time using AskUserQuestion:

### Architecture & Design
- What's the high-level architecture approach?
- How should components communicate?
- What are the key abstractions?
- Should this fail fast or retry with backoff?
- What's the error handling strategy?

### Implementation Details
- What libraries/frameworks should be used?
- What's the preferred code organization?
- Are there existing patterns to follow?
- What naming conventions?
- What's the testing approach?

### Data & State
- How is data structured?
- What's the state management approach?
- Where is data stored?
- What's the persistence strategy?

### Performance & Scale
- Expected load/traffic?
- Performance requirements?
- Caching strategy?
- Optimization priorities?

### Security & Privacy
- Authentication/authorization needs?
- Data sensitivity?
- Compliance requirements?

### Integration & Dependencies
- What external services?
- API specifications?
- Third-party dependencies?

### Deployment
- Where will this run?
- What's the deployment process?
- Environment configurations?

**Continue asking using AskUserQuestion until ZERO ambiguity remains.**

## Spec Document Structure

After complete interview, generate spec with:

1. **Feature Overview** - What and why
2. **Architecture Decision Records** - Every choice with rationale
3. **Technical Specification** - Detailed implementation plan
4. **API/Interface Contracts** - Clear boundaries
5. **Data Models** - Complete schemas
6. **User Flows** - Step-by-step scenarios
7. **Error Handling** - All edge cases
8. **Testing Strategy** - Unit, integration, e2e
9. **Implementation Phases** - Ordered steps
10. **Success Criteria** - Definition of done

## Final Instruction

**FINAL INSTRUCTION TO USER:**
"The spec is complete. Start a NEW SESSION and provide this spec as context. I'll implement it precisely because all ambiguity has been resolved upfront."

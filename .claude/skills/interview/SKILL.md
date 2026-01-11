---
name: interview
description: Interview me about the plan and write detailed spec
user-invocable: true
allowed-tools: Read, Write, Grep, Glob, AskUserQuestion
---

# Interview About Plan

Read the plan file and interview me **using the AskUserQuestion tool**.

**WORKFLOW (Spec-based development):**
1. Use AskUserQuestion to interview and gather all requirements
2. After interview is complete, write comprehensive spec document
3. User should start NEW SESSION with spec for execution

**IMPORTANT: You MUST use AskUserQuestion tool for every question. Do not proceed without user answers.**

## Interview Topics

Ask about (one question at a time using AskUserQuestion):
- Technical implementation details and approach
- UI/UX decisions and user flows
- Potential concerns and edge cases
- Tradeoffs and alternative approaches
- Performance implications
- Security considerations
- Testing strategy

Make sure the questions are:
- Not obvious - dig deep into the specifics
- Progressive - build on previous answers
- Technical - assume expertise
- Critical - challenge assumptions when needed

## Coverage Areas

Continue the interview using AskUserQuestion until you have a complete understanding of:
1. The full technical architecture
2. All major design decisions and their rationale
3. Identified risks and mitigation strategies
4. Clear acceptance criteria

## Spec Document Output

After the interview is complete, write a comprehensive spec document that includes:
- Executive summary
- Detailed technical specification
- Architecture diagrams (mermaid)
- Implementation phases
- Testing plan
- Open questions and risks

## Handoff

**IMPORTANT:** Tell the user to start a NEW SESSION with this spec for implementation.

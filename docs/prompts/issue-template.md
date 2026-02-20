# Issue Creation Prompt

Use this prompt with Claude to create well-structured GitHub issues.

---

Create a GitHub issue for a software project with the following format:

**Title:** `<type>: <short description>`
(use conventional commit types: feat, fix, perf, chore, docs, refactor)

**Description:**
2-3 sentences max. Explain the current problem or missing feature, why it matters,
and a brief hint at the solution approach. No implementation details, just enough
context to remember why this was flagged.

**Labels:** suggest 2 relevant labels (e.g. performance, enhancement, bug,
bastion-schema, technical-debt)

**Milestone:** suggest the appropriate version (v0.1.0, v0.2.0, v0.3.0...)
with a one-line justification for why it belongs there and not earlier or later.

---

Context about my project:
[describe your project briefly]

Issue to create:
[describe the problem or feature in your own words]

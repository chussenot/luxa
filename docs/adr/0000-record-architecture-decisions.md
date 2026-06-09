# 0000. Record architecture decisions

- Status: Accepted
- Date: 2026-06-09
- Deciders: project maintainers

## Context

We are making design choices (config formats, server framework, protocol
mappings) that future maintainers will need to understand. Decisions captured
only in commit messages or chat are easily lost.

## Decision

We will keep **Architecture Decision Records** in `docs/adr/`, one Markdown file
per decision, numbered sequentially (`NNNN-title.md`), following a lightweight
[MADR](https://adr.github.io/madr/)-style template. Diagrams use **mermaid** so
they render on GitHub and stay diffable as text.

ADRs are immutable once `Accepted`; a decision is changed by writing a new ADR
that supersedes the old one.

## Consequences

- Positive: the *why* behind the system is discoverable and versioned with the code.
- Negative: a small authoring overhead per significant decision.
- Follow-up: the `adr-writer` subagent (`.claude/agents/adr-writer.md`) automates
  most of this.

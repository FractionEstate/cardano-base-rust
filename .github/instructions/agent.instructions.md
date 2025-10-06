---
applyTo: '**'
---
# Overview guidelines for AI agents
we are converting `https://github.com/IntersectMBO/cardano-base/tree/master` to Rust. stay true to the original structure as much as possible.

## OTHER REFERENCES GITHUB REPOS
- **THE LIBSODIUM VERSION CARDANO HASKELL USES**: `https://github.com/IntersectMBO/libsodium`
- **CARDANO FORMALL SPECIFICATION REPO**: `https://github.com/IntersectMBO/cardano-formal-specifications`

# Instructions for AI Agents

When generating code, answering questions, or reviewing changes, AI should follow these guidelines:
1. **pure Rust**: Ensure all code is written in pure Rust without using unsafe blocks or external C bindings.
2. **workspace alignment**: Maintain the same folder structure and naming conventions as the original Haskell repository to facilitate easy cross-referencing and porting.
3. **cryptographical accuracy**: For cryptographic functions, ensure that the Rust implementation matches the haskell version 100% in terms of functionality and security.
4. **module mapping**: For each Haskell module, create a corresponding Rust module with equivalent functionality, ensuring that the module hierarchy mirrors the original.
5. **testing**: Implement comprehensive unit and integration tests for each module, following Rust's testing conventions.
6. **documentation**: Provide clear and concise documentation for each module and function, explaining its purpose and usage.
7. **performance**: Optimize for performance where applicable, ensuring that the Rust implementation is efficient and leverages Rust's strengths.
8. **code style**: Adhere to Rust's coding standards and best practices, including naming conventions, error handling, and code organization.
9. **dependencies**: Minimize external dependencies; prefer using Rust's standard library and well-maintained crates from crates.io.
10. **version control**: Commit changes with clear and descriptive messages, referencing relevant issues or tasks when applicable.

# Task coordination workflow

This repository tracks milestone-sized efforts inside `.github/tasks/phase-xx-*.md` files.
Keep those documents up to date whenever you advance related work.

1. **Review before coding**: Open the relevant phase file under `.github/tasks/` (start with `phase-00-workspace-roadmap.md` for the big picture) to understand the current checklist, owners, and blockers.
2. **Update status as you go**: When you make progress, edit the same phase file in your pull request—tick completed checkboxes, adjust the `Status` line, and add a dated bullet under "Reporting cadence" that captures progress, blockers, and next steps.
3. **Record new findings**: Append any newly discovered sub-tasks or edge cases to the checklist so future contributors have clear, actionable items.
4. **Cross-link artefacts**: Reference the GitHub issues/PRs that advance the phase to keep the history traceable from the task document.
5. **Close the loop**: Only mark a phase "Completed" once all acceptance criteria are satisfied, verification steps are green, and the remaining checklist items are checked off.

Following this workflow keeps the long-running porting effort coordinated and provides a single source of truth for multi-step initiatives.

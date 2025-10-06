---
applyTo: '**'
---
# Overview guidelines for AI agents
we are converting `https://github.com/IntersectMBO/cardano-base/tree/master` to Rust. stay true to the original structure as much as possible.

## OTHER REFERENCES GITHUB REPOS
- **THE LIBSODIUM VERSION CARDANO HASKELL USES**: `https://github.com/IntersectMBO/libsodium`
- **CARDANO FORMALL SPECIFICATION REPO**: `https://github.com/IntersectMBO/cardano-formal-specifications`

# Core obligations for AI contributors

Always satisfy every item below when generating code, answering questions, or reviewing changes. Do not proceed if any instruction is ambiguous—clarify or document assumptions.

## Implementation requirements

1. **Pure Rust only** – No `unsafe`, C FFI, or external bindings. Everything must compile with stable Rust.
2. **Workspace alignment** – Mirror the folder structure, module naming, and hierarchy used in the Haskell `cardano-base` repository so cross-referencing stays simple.
3. **Cryptographic fidelity** – For every crypto routine, match the Haskell behaviour exactly (bit-for-bit) and use vetted algorithms. Never change semantics unless the task explicitly demands it.
4. **Module mapping** – Create one Rust module per original Haskell module, maintaining the same relative path and public API surface wherever possible.
5. **Testing** – Ship comprehensive unit and integration tests in the crate’s `tests` or `src` modules. Cover happy paths, edge cases, and regression scenarios for any behaviour you touch.
6. **Debug/diagnostics parity** – Reuse the existing feature flags, environment toggles, trace helpers, and performance harness patterns already established in the workspace. Do not invent new ad-hoc logging schemes.
7. **Performance mindset** – Keep implementations efficient. Avoid needless allocations, copies, or dynamic dispatch unless justified. Benchmark when you change hot paths.

## Documentation expectations

1. **Per-crate README** – Every crate maintains a single `README.md`. Update it whenever functionality, usage, or dependencies change.
2. **Per-crate CHANGELOG** – Keep `CHANGELOG.md` up to date in each crate root. Summaries should mention new features, fixes, and breaking changes.
3. **Workspace README** – If work affects cross-crate behaviour, add or update the root `README.md` with links and explanatory notes.
4. **No extra docs** – Do **not** add new standalone documentation files (guides, notes, etc.). Fold all material into the existing README/CHANGELOG structure unless explicitly instructed otherwise.
5. **Reference provenance** – When you port behaviour from Haskell or another source, cite the original module or function in doc comments or README prose so future contributors can trace lineage easily.

## Dependency policy

1. Prefer the Rust standard library. Use third-party crates only when there is a strong justification (well-maintained, battle-tested, minimal footprint).
2. Additions to `Cargo.toml` must be accompanied by rationale in the relevant README or CHANGELOG.
3. Keep transitive dependency growth minimal; avoid optional features unless they are required.

## Version-control hygiene

1. Group related changes into coherent commits with descriptive messages.
2. Reference the associated task, issue, or phase document in commit messages or PR descriptions.
3. Keep diffs focused—no unrelated refactors or formatting churn.

# Task coordination workflow

# Task coordination workflow

This repository tracks milestone-sized efforts inside `/workspaces/cardano-base-rust/.github/tasks/.github/tasks/phase-xx-*.md` files.
Keep those documents up to date whenever you advance related work.

1. **Review before coding**: Open the relevant phase file under `/workspaces/cardano-base-rust/.github/tasks/` (start with `/workspaces/cardano-base-rust/.github/tasks/phase-00-workspace-roadmap.md` for the big picture) to understand the current checklist, owners, and blockers.
2. **Update status as you go**: When you make progress, edit the same phase file `/workspaces/cardano-base-rust/.github/tasks/.github/tasks/phase-xx-*.md` in your pull request—tick completed checkboxes, adjust the `Status` line, and add a dated bullet under "Reporting cadence" that captures progress, blockers, and next steps.
3. **Record new findings**: Append any newly discovered sub-tasks or edge cases to the checklist so future contributors have clear, actionable items.
4. **Cross-link artefacts**: Reference the GitHub issues/PRs that advance the phase to keep the history traceable from the task document.
5. **Close the loop**: Only mark a phase "Completed" once all acceptance criteria are satisfied, verification steps are green, and the remaining checklist items are checked off.

Following this workflow keeps the long-running porting effort coordinated and provides a single source of truth for multi-step initiatives.

# Pre-submit checklist for AI agents

Before finishing a task, confirm all the following:

1. ✅ Code adheres to pure Rust, workspace alignment, and module mapping rules.
2. ✅ Tests cover new or modified behaviour and pass locally.
3. ✅ Debug/diagnostics patterns follow the established feature-gated approach.
4. ✅ README/CHANGELOG (and root README if needed) document the changes.
5. ✅ Dependencies are justified or unchanged.
6. ✅ Relevant phase document under `/workspaces/cardano-base-rust/.github/tasks/phase-xx-*.md` is updated with progress.
7. ✅ Work-in-progress notes or learnings are captured directly in the README/CHANGELOG—no stray files.

# Prohibited actions

- ❌ Introducing `unsafe` blocks, C bindings, or non-Rust tooling.
- ❌ Rearranging the workspace structure away from the Haskell layout without explicit approval.
- ❌ Creating ad-hoc debug code, print statements, or logger setups outside the shared feature flags.
- ❌ Adding new documentation files outside the README/CHANGELOG framework.
- ❌ Pulling in large or unvetted dependencies for convenience.
- ❌ Skipping updates to phase trackers or READMEs when functionality changes.

Agents must stop and request clarification if an assignment conflicts with these rules.

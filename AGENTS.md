# Repository Guidelines

## Project Structure & Module Organization
The workspace is a single Rust crate defined by `Cargo.toml`. Core logic lives in `src/`, with `main.rs` wiring together the CLI (`cli.rs`), stack and registry helpers (`stack.rs`, `registry.rs`), and project/config management (`project.rs`, `config.rs`). CLI verbs map to implementations in `verbs.rs`. Add unit tests next to the code that exercises them and integration tests under `tests/` if end-to-end coverage is required.

## Build, Test, and Development Commands
Run every change through the standard cargo workflow:
- `cargo fmt` — formats Rust code with repository defaults.
- `cargo clippy --all-targets` — lints with Rust 2024 warnings enabled; fix or explicitly allow new lints.
- `cargo check` — type-checks quickly before commits.
- `cargo test` — executes unit/integration tests; add `-- --nocapture` when debugging CLI output.
- `cargo run -- <verb>` — invokes the CLI locally, e.g. `cargo run -- dev` or `cargo run -- resume project-x`.

## Coding Style & Naming Conventions
Code targets edition 2024 with rustfmt defaults (4-space indentation, max-width 100). Keep modules and files snake_case, structs/enums PascalCase, and functions snake_case. Clap-derived structs should keep descriptive field docs so `--help` stays informative. When adding configuration, persist it in `axl.toml` via the `ProjectConfig` helpers instead of ad-hoc file reads.

## Testing Guidelines
Use the built-in `#[cfg(test)] mod tests` pattern inside each module, mirroring the type or verb being tested (e.g., `handles_stack_resume`). Favor deterministic tests that stub filesystem paths rather than touching the user profile. Cover new verbs with command-spec parsing tests in `config.rs` and behavior tests for `verbs.rs`. Run `cargo test` locally before pushing; no release is accepted with failing or skipped suites.

## Commit & Pull Request Guidelines
History currently uses short, lowercase summaries (see `first commit`); continue writing concise, imperative subjects under 72 characters and add wrapped body paragraphs describing rationale and risks. Reference related issues (`Refs #123`) and note config impacts (e.g., migrations to `axl.toml`). Pull requests must include: summary of changes, testing evidence (`cargo test`, sample `cargo run -- verb` output), screenshots or logs for UX changes, and a checklist confirming docs such as this file remain accurate.

## Configuration & Environment Tips
Each project can opt into custom commands via `axl.toml`, with `[project]` metadata and verb-specific entries (command, `requires`, `env`). Use `ProjectConfig::load` to resolve paths inside `$PROJECT_ROOT`. Prefer the `directories` crate for cross-platform filesystem helpers.

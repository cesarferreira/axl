# AXL

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust 1.81+](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://www.rust-lang.org/)

**AXL** is the single muscle-memory for every repo you touch. It inspects the
current tree, figures out the stack (Android, Bun, Rust, Flutter, Bazel, …),
and maps universal verbs like `dev`, `build`, `test`, `reset`, or `logs` to the
right stack-specific commands. Instead of memorising 20 rituals, you just run:

```bash
axl dev      # start the dev server / run target
axl build    # produce release artifacts
axl test     # execute the stack's tests
axl clean    # remove build artifacts
axl reset    # wipe caches + reinstall deps
axl open     # open the project in the OS UI
axl logs     # tail relevant logs
axl install  # install the project locally
```

AXL is ideal for engineers juggling dozens of repositories and switching
between wildly different toolchains all day. It removes cognitive overhead
instead of adding yet another bespoke CLI to learn.

---

## Features

- **Stack auto-detection** — looks for marker files such as `bun.lock`,
  `package.json`, `gradlew`, `Cargo.toml`, `go.mod`, `Gemfile`, `pom.xml`,
  `composer.json`, `mix.exs`, `*.csproj`, `Package.swift`, `pubspec.yaml`, 
  `BUILD.bazel`, `pyproject.toml`, or falls back to `.git`.
- **Zero-config defaults** — each stack ships with sensible commands for all
  verbs (e.g. Bun → `bun run dev`, Rust → `cargo run`, Go → `go run .`). Works out-of-the-box.
- **Optional overrides** — drop an `axl.toml` in any repo to redefine verbs,
  set env vars, enforce tool requirements, or change the detected stack.
- **Unified helper verbs** — beyond the core runtime verbs you also get
  `axl info`, `axl detect`, `axl doctor`, `axl init`, `axl recent`, `axl resume`,
  `axl switch`, and `axl version`.
- **Workspace registry** — remembers the last projects you touched, which verb
  you ran, and when. Stored in `~/.config/axl/projects.json`, enabling
  `axl recent`, `axl resume`, and `axl switch`.
- **Safety checks** — every command can declare required tools (e.g. `bun`,
  `cargo`, `adb`, `go`, `dotnet`). AXL verifies they exist before spawning anything.

---

## Installation

```bash
cargo install --path .
```

(Or build however you prefer; it’s a single Rust binary.)

---

## Usage

From any repository:

```bash
axl dev      # start the dev server / run target
axl build    # produce release artifacts
axl test     # execute the stack's tests
axl clean    # remove build artifacts
axl reset    # wipe caches + reinstall deps
axl open     # open the project in the OS UI
axl logs     # tail relevant logs
axl install  # install the project locally
```

| Verb         | Description                                                     |
| ------------ | --------------------------------------------------------------- |
| `axl dev`    | Launch the stack's "dev" target (server, emulator, binary, etc.)|
| `axl build`  | Produce release/production artifacts for the current project.   |
| `axl test`   | Run the default test suite for the detected stack.              |
| `axl clean`  | Remove build outputs / caches to get a fresh tree.              |
| `axl reset`  | Perform a deeper nuke (node_modules, Gradle caches, bootstrap). |
| `axl open`   | Open the repo in the OS UI (IDE, Finder, Explorer, etc.).       |
| `axl logs`   | Tail the relevant logs (adb logcat, flutter logs, vite output). |
| `axl install`| Install the project locally (cargo install --path ., etc.).     |

### Project helpers

| Command          | Description                                                      |
| ---------------- | ---------------------------------------------------------------- |
| `axl info`       | Show detected stack, config file, and the resolved command map.  |
| `axl detect`     | Print only the detection reason (handy for debugging).           |
| `axl doctor`     | Check for missing requirements per verb.                         |
| `axl init`       | Generate an `axl.toml` with stack defaults (use `--force` to overwrite). |

### Workspace helpers

| Command               | Description                                                |
| --------------------- | ---------------------------------------------------------- |
| `axl recent`          | List recent projects + last verb/time.                     |
| `axl resume [query]`  | Jump back into the last verb for a project (fuzzy name).   |
| `axl switch [query]`  | Print `cd <path>` (or just the path with `--path-only`).   |

---

## Configuration (`axl.toml`)

AXL doesn’t need configuration, but when you want full control you can create
an `axl.toml` in the project root:

```toml
[project]
name = "polygone"
stack = "bun"

[dev]
cmd = "bun run dev"
requires = ["bun", "node"]

[test]
cmd = "bun run test"

[reset]
cmd = "rm -rf node_modules bun.lock && bun install"
```

Each verb section can define:

- `cmd` (string): the exact command to run.
- `requires` (array): binaries that must exist in `$PATH`.
- `env` (table): environment variables to inject when running.

If `[project]` supplies `stack`, it overrides detection.

---

## Detection table (built-in markers)

| Marker           | Stack / defaults                    |
| ---------------- | ----------------------------------- |
| `bun.lock`       | Bun                                 |
| `package.json`   | Node / TypeScript                   |
| `gradlew`        | Gradle / Android / Kotlin           |
| `Cargo.toml`     | Rust                                |
| `pubspec.yaml`   | Flutter                             |
| `melos.yaml`     | Dart monorepo (Melos)               |
| `BUILD.bazel`    | Bazel                               |
| `pyproject.toml` | Python                              |
| `go.mod`         | Go                                  |
| `Gemfile`        | Ruby / Rails                        |
| `pom.xml`        | Maven / Java                        |
| `composer.json`  | PHP / Laravel                       |
| `mix.exs`        | Elixir / Phoenix                    |
| `*.csproj`       | .NET / C#                           |
| `*.sln`          | .NET / C# (solution)                |
| `Package.swift`  | Swift                               |

If `axl.toml` exists, its `stack` value wins.

---

## Development

```
cargo fmt
cargo check
cargo run -- dev      # or any other verb
```

AXL sets `AXL_VERB` and `AXL_ROOT` when spawning commands so scripts can
inspect the context if needed.

---

## License

[MIT](LICENSE)

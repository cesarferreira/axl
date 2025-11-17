

# 🔥 THE CORE IDEA OF **AXL**

You operate inside a constant storm of different stacks:

* Android / Gradle / Kotlin / Java
* Flutter / Dart / Melos
* Bun / Node / TypeScript
* Rust / Cargo
* Python
* Bazel
* Your weird ecosystem of robin, fp, adb, etc.

Each one has its own:

* dev commands
* build commands
* test commands
* clean/reset rituals
* environment traps
* tooling quirks
* boilerplate scripts
* mocking flows
* logging flows

You context-switch between repos like a machine gun.

Each repo requires you to remember **a whole ritual vocabulary** just to get anything running.

AXL deletes all that mental load.

```
axl dev
axl build
axl test
axl reset
axl clean
axl open
axl logs
```

AXL **automatically knows what these verbs mean** for the project you’re in — no matter the stack.

This is the first CLI that reduces cognitive overhead instead of adding to it.

---

# 💡 WHO **AXL** SERVES

* Engineers juggling **10–20 active repos**
* People using **multiple languages** daily
* People bouncing between **wildly different toolchains**
* People maintaining both **work + side projects**
* Anyone tired of remembering 50 inconsistent commands
* Anyone who rebuilds environments often
* You

This isn’t for beginners.
This is for someone who values **velocity** and hates busywork.

---

# ⚙️ HOW **AXL** THINKS

## **1. Auto-detect the project type**

AXL scans the current directory and finds markers:

| File / Folder    | Stack                     |
| ---------------- | ------------------------- |
| `package.json`   | Node / TS                 |
| `bun.lockb`      | Bun                       |
| `gradlew`        | Gradle / Android / Kotlin |
| `Cargo.toml`     | Rust                      |
| `pubspec.yaml`   | Flutter                   |
| `melos.yaml`     | Dart monorepo             |
| `BUILD.bazel`    | Bazel                     |
| `.robin.json`    | Wayve tooling             |
| `pyproject.toml` | Python                    |
| `.git`           | Generic fallback          |

If there’s an `axl.toml`, that wins.
If not, detection gives you correct defaults 80% of the time.

---

## **2. Load local command profiles**

Each project optionally defines a config:

`axl.toml`

```toml
[dev]
cmd = "bun run dev"
requires = ["node", "bun"]

[build]
cmd = "bun run build"

[test]
cmd = "bun run test"

[reset]
cmd = "rm -rf node_modules bun.lockb && bun install"
```

If this file doesn’t exist, AXL uses its own defaults for the detected stack.

You get the best of both worlds:

* Zero-config for simple repos
* Full override control for complex ones

---

## **3. AXL maps global verbs → local commands**

Regardless of stack, these verbs always exist:

```
axl dev
axl build
axl test
axl clean
axl reset
axl open
axl logs
```

Every project obeys the same contract.
Consistency kills friction.

---

## **4. (Optional) Smart mode**

Not “AI”.
Just intelligent observation:

* If you ran `bun run dev` 60 times → AXL promotes it to default.
* If you always follow `gw clean` with `gw assembleDebug` → AXL groups them into a recipe.
* If you always run `robin dev:start` inside Android repos → AXL autogenerates that mapping.

This is **frequency-driven** behaviour, not guesswork.

---

## **5. Universal log tailer**

```
axl logs
```

Depending on the stack, this auto-tails:

* `adb logcat` (Android)
* `bun run dev` logs
* `cargo run` output
* `flutter run` output
* `docker compose logs`
* `robin dev:start` logs

No configuration.
AXL knows what the project “is”.

---

## **6. Workspace registry**

AXL keeps simple metadata in:

`~/.config/axl/projects.json`

It tracks:

* path
* project name
* stack
* last-used verb
* last-used timestamp

That gives you:

```
axl recent
axl resume
axl resume polygone
axl switch
axl open robo
```

You stop manually remembering where you last were.

---

# 🔥 WHY **AXL** MATTERS

Look at your actual workflow:

* Recalling 20 different dev commands
* Rebuilding broken environments
* Switching between Android, Bun, Rust, Flutter
* Running long mirror commands like `gw assembleCanaryDevDebug`
* Repeating build–clean–reset cycles
* Searching README files for “how do I run this again?”
* Copy-pasting the same commands to chat or notes
* Wasting your brain on useless glue steps

AXL erases all of this.
One interface.
Across all repos.
Across all stacks.

It gives you **one place for your hands to go** for all dev actions.

---

# 🧪 REAL USAGE FOR *YOU*

### **Android (robot-android)**

```
cd robo
axl dev       → robin dev:start
axl build     → gw assembleCanaryDevDebug
axl logs      → adb logcat | grep relevant tags
axl reset     → wipes gradle caches + daemon
```

### **Polygone (Bun)**

```
cd polygone
axl dev       → bun run dev
axl test      → bun run test
axl build     → bun run build
axl logs      → vite output
```

### **QuickMaths (Rust)**

```
axl dev       → cargo run
axl test      → cargo test
```

### **Echoes (Swift/macOS)**

```
axl build     → xcodebuild
axl open      → open Xcode project
axl logs      → macOS console filters
```

AXL doesn’t need detailed config to do all this.
It infers.

---

# 🏷 NAME IS NOW **AXL**

Strong.
Short.
Fast.
QWERTY-friendly.
You’ll type it hundreds of times a week and not hate it.

**AXL = the axle your workflow rotates around.**
Everything stays centered on it.

---


also have:
```
axl info
axl detect              # debug: show what AXL thinks this project is
axl recent              # list recent projects
axl resume [<project>]  # jump back into last/selected project+verb
axl doctor              # common problems

axl init                # create a default axl.toml in this repo
axl version             # show AXL version
axl help                # show help
```

by default it auto detect the project and use the default commands for the project.
if you want to override the default commands for a project, you can create a axl.toml file in the root of the project. and override the default commands for the project.
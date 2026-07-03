# Releasing a New Version

This project automates the release process using `cargo-release` and `git-cliff`. 

When a release is triggered, the tools automatically:
1. Bump the versions in all `Cargo.toml` manifests.
2. Synchronize internal workspace dependencies.
3. Automatically update version references in source files (`src/lib.rs`) and documentation (`README.md`).
4. Prepend the newest release notes to `CHANGELOG.md` (leaving older manual entries untouched).
5. Create a signed Git commit and tag.
6. Publish both crates to [crates.io](https://crates.io) in the correct order.
7. Trigger GitHub Actions to create an official GitHub Release with beautiful release notes.

---

## Prerequisites

Before releasing for the first time, ensure you have the necessary tools installed on your machine:

```sh
# Install cargo-release
cargo install cargo-release

# Install git-cliff
cargo install git-cliff
```

You must also be authenticated with `crates.io` locally:
```sh
cargo login <your-crates-io-api-token>
```

---

## De Release Command Syntax

You will use `cargo release <level>` to define what kind of release you are making, based on [Semantic Versioning](https://semver.org/):

* **`patch`** (e.g., `0.5.2` -> `0.5.3`): For backwards-compatible bug fixes.
* **`minor`** (e.g., `0.5.2` -> `0.6.0`): For backwards-compatible new features.
* **`major`** (e.g., `0.5.2` -> `1.0.0`): For breaking API changes.

---

## Step-by-Step Release Guide

### Step 1: Prepare
Ensure you are on the `main` branch, it is up-to-date, and all tests pass locally:
```sh
git checkout main
git pull origin main
cargo test
```

### Step 2: The Dry Run (Safety First! 🛡️)
Always simulate the release first to ensure that there are no compilation issues, test failures, or version-sync discrepancies. This step will **not** change any files or push anything.

Run the command corresponding to your change level:
```sh
# For a patch release
cargo release patch --dry-run

# For a minor release
cargo release minor --dry-run

# For a major release
cargo release major --dry-run
```
*Carefully read the output in the terminal. Ensure the version replacements in `Cargo.toml`, `README.md`, and `src/lib.rs` are executed correctly.*

### Step 3: Execute the Release 🚀
If the dry run succeeded and looks perfect, execute the actual release. 

**Note:** This command will modify files, commit, tag, push to GitHub, and publish to `crates.io`.

```sh
# Choose the correct command:
cargo release patch --execute
# OR
cargo release minor --execute
# OR
cargo release major --execute
```

---

## What Happens Automatically Next?

1. **GitHub CI (`build-ci`):** Will run the test suite and a security audit on the new commit.
2. **GitHub Release (`release`):** 
   * The pushed git tag (e.g., `v0.5.3`) automatically triggers the release workflow.
   * `git-cliff` compiles the release notes specifically for this new tag.
   * A draft-free, public GitHub Release is created automatically with your beautiful changelog.

# zbus-lockstep Changelog

All notable changes to this project will be documented in this file.

## [0.6.0] - 2026-07-07

This release documents how to use the crate as `[dev-dependency]`.
The `validate` macro will return early when env variable `LOCKSTEP_XML_PATH` is found.
Introduces `macros` feature which re-exports the `validate` proc-macro,
allowing users to omit the `zbus-lockstep-macros` dependency.

### ✨ Features

- Feat (main)✨: Introduce `macros` feature ([40a3aa9](https://github.com/luukvanderduim/zbus-lockstep/commit/40a3aa98f08c530f21d5afa306a9c98a4a66a6bc))

### ♻️ Refactor / Move files

- Refactor (main)♻️: No more panics inside `resolve_xml_path` ([09e1e67](https://github.com/luukvanderduim/zbus-lockstep/commit/09e1e67af572d7b6f57fc394252073fd2ef78b41))
- Refactor (macros) ➖: Do not depend on zbus-locstep ([0cdebf6](https://github.com/luukvanderduim/zbus-lockstep/commit/0cdebf65997abf68221f332b168eda35f27a415b))

### Build

- *(⬆️)* Keep `dev-dependencies` latest ([955a5ce](https://github.com/luukvanderduim/zbus-lockstep/commit/955a5ce61430050d18a55ccd0513c80f4ff0170a))

### ✅ Testing and validation

- Tests (macros) ✅: Raise doc `ignore` blocks to tests ([9440a61](https://github.com/luukvanderduim/zbus-lockstep/commit/9440a61a232fab208660230565f4375b5673e0d7))

## [zbus-lockstep-v0.5.2] - 2025-11-29

chore: Clippy fixes (#8 Tait Hoyem)

## [zbus-lockstep-v0.5.1] - 2025-06-24

feat: make `[validate]` attribute more flexible (#6) Tait Hoyem
     * Add support for any type - no longer restrict to structs.

build: Support bazel - (#5) by Nils Andre
    use CARGO_MANIFEST_DIR instead of current dir.

## [zbus-lockstep-v0.5.0] - 2024-12-02

### Update dependencies

- Move to zbus 5.0 (by TTWNO)

## [zbus-lockstep-v0.4.4] - 2024-03-18

### 🧑‍💻 Improve DX

- 🧑‍💻 Also look in the directory with the crate name.
     This is useful if build is invoked from the workspace.

## [zbus-lockstep-v0.4.3] - 2024-03-18

### 🐛 Fixes  

- 🩹 `validate` now requires a comma separated argument list if multiple arguments are supplied.
- 🩹 In macros prefer relative paths to absolute paths.

### 📝 Documentation

- 📝 Mention `resolve_xml_path` can now find `..xml/` or `..XML/` too.

### 🧑‍💻 Improve DX

- 🧑‍💻 Be more lenient by adding parent to defaults for `resolve_xml_path`

## [zbus-lockstep-v0.4.2] - 2024-03-09

## [zbus-lockstep-v0.4.2] - 2024-03-09

### 🚸 Improve UX

- 🚸 Remove requirement `zbus_xml` to be in scope at macro call site.  
- Add `Copy`, `Clone`, `Hash`, to public `MshType`.

## [zbus-lockstep-v0.4.1] - 2024-03-05

### 📝 Documentation

- 📝 Update Readme to `zbus-xml` instead of `zbus`

### ✅ Testing and validation

- ✅ [lockstep] Remove unnecessary references.  
- ✅ [macros] validate generated test now decorated with `#[cfg(test)]`

## [zbus-lockstep-v0.4.0] - 2024-02-19

### ⚙️ Miscellaneous Tasks

- ➖ Swap zbus for zbus_xml
- ➖ Remove `signatures_are_eq`, `assert_signatures_eq!` and
`assert_signatures_ne!`.

- ➖ Remove tests for  `signatures_are_eq`, `assert_signatures_eq!` and
`assert_signatures_ne!`.

## [zbus-lockstep-v0.3.1] - 2023-10-02

### ⏪️ Revert / Remove

- 🔥 Cargo.lock
- ➖ Do not add changed Cargo.lock

## [zbus-lockstep-v0.3.0] - 2023-09-28

### ✨ Features

- ✨ `property_type_signature` macro accepts identifying args.

```rust
property_type_signature!(member: "Frequency", interface: "org.noise.Kazoo")
```

- ✨ `signal_body_type_signature` macro accepts identifying args.

```rust
signal_body_type_signature!(member: "Telegram", interface: "org.mail.Man")
```

- ✨ `method_args_signature` macro accepts identifying args.

```rust
 method_args_signature!(member: "Frequency", interface: "org.noise.Kazoo")
```

- ✨ Add `argument` parameter to `method_return_signature` macro.
- ✨ Add `argument` parameter to `method_args_signature` macro.
- ✨ Add `argument` parameter to `signal_body_type_signature` macro.

### 🐛 Fixes

- 🐛 Cannot disambiguate between multiple 2-arg macro invocations

Remove second 2-arg macro invocations.

### 📝 Documentation

- 📝 `property_type_signature` macro doc examples.
- 📝 `signal_body_type_signature` macro doc examples.
- 📝 `method_return_signature` macro doc examples.
- 📝 Update docs to accompany added `argument` feature
New examples to show how the macros may be called.

### ✅ Testing and validation

- ✅ Add `property_type_signature` macro identifier unit tests.
- ✅ Add `signal_body_signature` macro identifier unit tests.
- ✅ Add `method_args_signature` macro identifier unit tests.
- ✅ Add `method_return_signature` macro identifier unit tests.
- ✅ Add six unit tests to cover 'happy path' in macros.

## [zbus-lockstep-v0.2.3] - 2023-09-24

### 🐛 Fixes

- 🐛 In macros with 'interface' arms, pass `String` instead of `&str`
- 🐛 'get_property_type' expects three args, not four.

### ✅ Testing and validation

- ✅ Add tests for macros with 'interface' arms.

## [zbus-lockstep-v0.2.2] - 2023-09-23

### 🐛 Fixes

- 🩹 Fully qualify `signatures_are_eq` in macros.

## [zbus-lockstep-v0.2.1] - 2023-09-22

### 🐛 Fixes

- 🩹 Fully qualify `find_definition_in_dbus_xml!` in macros.

### ✅ Testing and validation

- ✅ Ignore examples from test
- ✅ Adds tests for XML path retrieval macros:

- `method_return_signature`
- `method_args_signature`
- `signal_body_type_signature`
- `property_type_signature`

## [zbus-lockstep-v0.2.0] - 2023-08-31

### ✨ Features

✨ Add signature retrieval macros for each msg-type

- `resolve_xml_path` function
- `find_definition_in_dbus_xml` Is used in macros:
- `method_return_signature`
- `method_args_signature`
- `property_type_signature`
- `signal_body_type_signature`

### 🐛 Fixes

- 🐛 Fix falsely identified previously found signal.
- Remove senseless checks.
- 🐛 [macros] Do not treat subdirectories as files.
- 🐛 Skip subdirectories in the xml-definitions path.
- 🐛 [macros] Cannot read contents from `Cursor<str>`, read from a `File`.
- 🐛 Canonicalize env and argument provided paths alike.
- 🐛 Fix: call the generated test after it is generated.
- 🩹 Replace hard coded directory paths with consts

### 📝 Documentation

- 📝 [macro] Improve `README.md`
- 📝 [macros] Improve `validate` macro documentation
- 📝 [macros] Document order of attribute macros
- - Remove mention of irrelevant `Cache.xml`
- 📝 Synchronize the three README's

### ♻️ Refactor / Move files

- ♻️ [macros] Remove function that resolves xml
- 🚚 Remove module `marshall` from public API

### ✅ Testing and validation

- ✅ [macros] CI test for path as argument
- ✅ [macros] Ads tests for `RemoveNode` with an xml-path as env-variable or as
argument.
- ✅ Failing test in GH

### ⚙️ Miscellaneous Tasks

- Remove dependency on self in dev-dependencies

## [0.1.0] - 2023-08-08

### 📝 Documentation

- 📝 Add CI badge to README.md

### ✅ Testing and validation

- ✅ Add example to `get_method_args_type`
- ✅ version-numbers tests

### 🎉 Begin project

- 🎉 Initial commit

First commit for helper crate `zbus-lockstep`

<!-- generated by git-cliff -->

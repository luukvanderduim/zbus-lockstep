# zbus-lockstep Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

---

## [zbus-lockstep-v0.4.3] - 2023-03-18

### 🐛 Fixes  

- 🩹 `validate` now requires a comma separated argument list if multiple arguments are supplied.
- 🩹 In macros prefer relative paths to absolute paths.

### 📝 Documentation

- 📝 Mention `resolve_xml_path` can now find `..xml/` or `..XML/` too.

### 🧑‍💻 Improve DX

- 🧑‍💻 Be more lenient by adding parent to defaults for `resolve_xml_path`

## [zbus-lockstep-v0.4.2] - 2023-03-09

### 🚸 Improve UX

- 🚸 Remove requirement `zbus_xml` to be in scope at macro call site.  
- Add `Copy`, `Clone`, `Hash`, to public `MshType`.

## [zbus-lockstep-v0.4.1] - 2023-03-05

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

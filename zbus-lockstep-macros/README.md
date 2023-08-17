# zbus-lockstep-macros

[![crates-io](https://img.shields.io/crates/v/zbus-lockstep-macros.svg)](https://crates.io/crates/zbus-lockstep-macros)
[![api-docs](https://docs.rs/zbus-lockstep-macros/badge.svg)](https://docs.rs/zbus-lockstep-macros)

More conveniently keep type definitions in in lockstep with DBus XML descriptions - with [`zbus`](<https://github.com/dbus2/zbus>) and [`zbus-lockstep`](<https://github.com/luukvanderduim/zbus-lockstep/zbus-lockstep>).

This extends `zbus-lockstep` to more succinctly and conveniently match the signature of [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) with a corresponding signature from a DBus XML file.

See for the motivation the `zbus-lockstep` crate.

## To-do

- [ ] Provide proc-macro to derive a validation

```rust
#[derive(Type)] 
#[validate(signal = "Activate", path = "../xml/introspected.xml")]
pub struct ActivateEvent {
    event: String,
    serial: u32,
    // 
}
```

## LICENSE

MIT

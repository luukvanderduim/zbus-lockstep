# zbus-lockstep-macros

[![CI](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/zbus-lockstep.svg)](https://crates.io/crates/zbus-lockstep)
[![api-docs](https://docs.rs/zbus-lockstep/badge.svg)](https://docs.rs/zbus-lockstep)

More conveniently keep type definitions in in lockstep with DBus XML descriptions using [`zbus`](<https://github.com/dbus2/zbus>) and [`zbus-lockstep`](<https://github.com/luukvanderduim/zbus-lockstep/zbus-lockstep>).

This extends `zbus-lockstep` to more succinctly and conveniently match the signature of [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) with a corresponding signature from a DBus XML file.

See for the motivation the [`zbus-lockstep`](https://github.com/luukvanderduim/zbus-lockstep/zbus-lockstep) crate.

## Use

Add `zbus-lockstep-macros` to `Cargo.toml`'s dev-dependencies:

```toml
[dev-dependencies]
zbus-lockstep-macros = "0.1.0"
```

If the `DBus` XML descriptions can be found in the crates root,
in either `xml/` or `XML/`, validating the type can be as easy as:

```rust
 use zbus_lockstep_macros::validate;
 use zbus::zvariant::Type;

 #[validate]
 #[derive(Type)]
 struct BirthdayEvent {
    name: String,
    new_age: u8,
}
```

Note that the macro assumes that the member name is contained in the struct name.
You can provide it you have another naming-scheme in use.

Also, it may be necessary to disambiguate if multiple interfaces across the `DBus`
descriptions provide equally named signals.

Any of the arguments are optional.

`#[validate(xml: <xml_path>, interface: <interface_name>, member: <member_name>)]`

## LICENSE

MIT

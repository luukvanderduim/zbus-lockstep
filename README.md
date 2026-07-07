<div align="center">

# zbus-lockstep

[![CI](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

</div>

---

## Table of Contents

- [Motivation](#motivation)
- [Import](#import)
- [Usage](#usage)
- [Features](#features)
- [Test-gating `validate`](#test-gating-validate)
- [Note](#note)
- [Acknowledgement](#acknowledgement)
- [License](#license)

---

This repository is home to `zbus-lockstep` and `zbus-lockstep-macros`.

These crates help keep type definitions in lockstep with DBus XML descriptions, using [`zbus-xml`](<https://github.com/dbus2/zbus>).

These offer means to match your type's signature - [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) - with a corresponding signature retrieved from a DBus XML file.

This way `zbus-lockstep` and `zbus-lockstep-macros` prevent definitions from drifting apart.

## Motivation

In the context of IPC over `DBus`, especially where there are multiple implementations of servers and/or clients communicating, it is necessary for each implementation to send what others expect and that expectations are in accordance with what is sent over the bus.

The `XML` protocol-descriptions may act as a shared frame of reference or "single source of all truth" for all implementers.
Having a single point of reference helps all implementers meet expectations on protocol conformance.

Keeping the types you send over `DBus` in lockstep with currently valid protocol-descriptions will reduce chances of miscommunication or failure to communicate.

## Import

Add the `zbus-lockstep` crate to your `Cargo.toml` with the `macros` feature enabled:

```toml
[dev-dependencies]
zbus-lockstep = { version = "0.5.2", features = ["macros"] }
```

or, from your shell:

```shell
cargo add zbus-lockstep --dev --features macros
```

## Usage

Consider the followwing XML description:
An interface with a single signal in the `Node.xml` file.

```XML
<node>
  <interface name="org.example.Node">

    <signal name="RemoveNode">
      <arg name="nodeRemoved" type="(so)"/>
    </signal>

  </interface>
</node>
```

The associated Rust type might look like this:

```rust
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

The derive macro in this example implements the [`zvariant::Type`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html).
This means we can now call `<Node as Type::signature()`, which will return a [`zvariant::Signature`](https://docs.rs/zvariant/latest/zvariant/struct.Signature.html) of the type.

The test below shows how `zbus-lockstep` may be used to assure our implementation is indeed in lockstep with the protocol descriptions.

```rust
    use zbus_lockstep;

    #[test]
    fn test_get_signature_of_cache_remove_accessible() {
        let xml = PathBuf::from("xml/Node.xml");
        let iface = "org.example.Node";
        let member = "RemoveNode";

        let signature = get_signal_body_type(xml, iface, member, None).unwrap();
        assert_eq!(signature, Node::signature());
    }
```

This common case can also be handled even more succinctly by `zbus-lockstap-macros::validate`, shown below:

```rust
use zbus_lockstep_macros::validate;

#[validate]
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

Which does essentially the same as the previous example; it creates a test that validates whether the signature of the `RemoveNode` signal matches the `Node` type.

See either crate and their docs for more details on usage and options.

## Features

- `macros`: Enables the `zbus-lockstep-macros::validate` proc-macro re-export from `zbus-lockstep`.

```toml
[dependencies]
zbus-lockstep = { version = "0.5.2", features = ["macros"] }
```

## Test-gating `validate` 

Users may want to consider gating the `validate` proc-macro to avoid compile-time file I/O during production builds.
`zbus_lockstep_macros::validate` notifies users when an XML path cannot be found or whether a corresponding definition
cannot be found for the Rust type. This requires the macro to look for, open, and read the XML files.

By gating the macro with `#[cfg_attr(test, ...)]`, it will only be invoked when building tests:

```rust
// Note: Cargo automatically translates the hyphen in 'zb-lsm' to an underscore 'zb_lsm' in Rust code.
#[cfg_attr(test, zbus_lockstep::validate)]
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

Now that `validate` is only built when the cargo test profile is active, 
users can safely move the dependency into `[dev-dependencies]`.

```toml
[dev-dependencies]
zbus-lockstep = { version = "0.5.2", features = ["macros"] }
```

See this in action in [the minimal end-use example.](https://github.com/luukvanderduim/zbus-lockstep/tree/main/e2e/lockstep_user)

## Note

When using XML descriptions as point of reference, you should ensure that the descriptions in use are always the most recent available.

Automated synchronizing would be preferred.

## Acknowledgement

`zbus-lockstep` started out as a fork of Tait Hoyem's [zbus-xml-match](https://github.com/TTWNO/zbus-xml-match).

## LICENSE

MIT

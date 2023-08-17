# zbus-lockstep and zbus-lockstep-macros

[![CI](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/zbus-lockstep.svg)](https://crates.io/crates/zbus-lockstep)
[![api-docs](https://docs.rs/zbus-lockstep/badge.svg)](https://docs.rs/zbus-lockstep)

This repository is home to `zbus-lockstep` and `zbus-lockstep-macros`.

These crates help keep type definitions in lockstep with DBus XML descriptions using [`zbus`](<https://github.com/dbus2/zbus>).

These provide means to match the signature of [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) with a corresponding signature from DBus XML.

`zbus-lockstep` and `zbus-lockstep-macros` help prevent type definitions drift by offering functions and an attribute macro to retrieve and assert that the signature of the type corresponds to signature in the XML-description.

## Motivation

In the context of IPC over `DBus`, especially where there are multiple implementations of servers and/or clients communicating, it is necessary for each implementation to match each others expectations.

The `XML` descriptions may act as a shared overarching frame of reference or "a single source of all truth".
Having a single point of reference helps all implementers meet expectations on protocol conformance.

## How

Consider the followwing XML description,
an interface with a single, simple signal in the `Cache.xml` file:

```XML
<node>
  <interface name="org.example.Node">

    <signal name="RemoveNode">
      <arg name="nodeRemoved" type="(so)"/>
    </signal>

  </interface>
</node>
```

The type in our implementation might look like this:

```rust
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

The derive macro in this example implements the [`zvariant::Type`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html).
This means we can now call `<Node as Type::signature()`, which will return a [`zvariant::Signature`](https://docs.rs/zvariant/latest/zvariant/struct.Signature.html) of the type.

The test below shows how `zbus-lockstep` may be used.

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

In the case above we are testing whether a signal body type corresponds to our signal or event type.
This common case can be handled even more succinctly by `zbus-lockstap-macros::validate`.

```rust
use zbus_lockstep_macros::validate;

#[validate]
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

Which does essentially the same as the previous example.

The implementer is required to ensure that the `XML` descriptions that are in use,
are indeed the most recent available.

See either crate for more details on usage and options.

## LICENSE

MIT

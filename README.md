# zbus-lockstep

[![build-ci](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)

Keep type definitions in lockstep with DBus XML descriptions, with [`zbus`](<https://github.com/dbus2/zbus>).

This provides means to match the signature of [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) with a corresponding signature from a DBus XML file.

`zbus-lockstep` prevents definitions from drifting apart by offering means to retrieve and assert that the signature of a type
corresponds to signature in the currently valid `XML`.

## Why

To ensure that a type's implementation matches a signature in an external XML DBus description.

In the context of IPC over `DBus` - especially where are multiple implementations of servers and/or clients - it is necessary for each implementation to match what others expect.
The `XML` descriptions may act as a shared overarching frame of reference or "single source of all truth". Having a single point of reference helps all implementers meet expectations on protocol conformance.

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

The test below shows how `zbus-lockstep` may be used given what we know about the type.

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

Obviously, the user here needs to take care and ensure that the `XML` descriptions that are in use,
are valid and the most current available.

## ToDo

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

Caveat: Requires a sub-crate.

## Acknowledgement

This crate started out as a fork from Tait Hoyem's [zbus-xml-match](https://github.com/TTWNO/zbus-xml-match).

## LICENSE

MIT OR APACHE-2.0

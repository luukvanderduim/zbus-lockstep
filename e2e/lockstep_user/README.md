# lockstep-user 

Demonstrates how to use `zbus-lockstep-macros` as dev-dependency.

The `validate` macro here is gated by cargo profile `test`.
This causes `validate` to generate a test only in test builds.

Run `cargo test` to run the generated test.

### Compile time checks

Note that `validate`'s compile time checks on arguments and XML 
path resolvability now also move to test context.

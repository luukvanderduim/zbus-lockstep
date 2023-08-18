// tests/attribute_macro.rs
#![allow(unnameable_test_items)]

use zbus::zvariant::{OwnedObjectPath, Type};
use zbus_lockstep_macros::validate;

#[test]
fn test_validate_macro_env_variable_set() {
    // set env variable to enable validation
    std::env::set_var("ZBUS_LOCKSTEP_XML_PATH", "zbus-lockstep-macros/tests/xml");

    #[validate]
    #[derive(Debug, Type)]
    struct AddNodeEvent {
        _name: String,
        _path: OwnedObjectPath,
    }
}

#[test]
fn test_validate_macro_t() {
    #[validate(xml: "zbus-lockstep-macros/tests/xml")]
    #[derive(Debug, Type)]
    struct AddNodeEvent {
        _name: String,
        _path: OwnedObjectPath,
    }
}

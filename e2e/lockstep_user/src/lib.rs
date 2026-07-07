#![allow(dead_code)]
use zvariant::Type;

#[cfg_attr(test, zbus_lockstep::validate(xml: "xml"))]
#[derive(Type)]
pub struct Alert {
    urgent: bool,
    color: String,
    volume: f64,
}

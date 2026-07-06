#![allow(dead_code)]
use zvariant::Type;

#[cfg_attr(test, zb_lsm::validate(xml: "xml"))]
#[derive(Type)]
pub struct Alert {
    urgent: bool,
    color: String,
    volume: f64,
}

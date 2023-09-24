#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{fs, path::PathBuf, str::FromStr};

use zbus::xml;

use crate::Result;

/// Resolve XML path from either:
///
/// - provided argument,
/// - default location (`xml/` or `XML/`) or
/// - env_variable (`LOCKSTEP_XML_PATH`)
///
/// If no XML path is provided, it tries to find the default XML path.
/// If the environment variable is set, it overrides the default, or
/// argument path.
///
/// # Example
///
/// ```rust
/// # use zbus_lockstep::resolve_xml_path;
/// # use std::path::PathBuf;
/// # fn main() {
/// // path to XML files
/// std::env::set_var("LOCKSTEP_XML_PATH", "../xml");
///
/// let xml_path = resolve_xml_path(None).unwrap();
/// assert_eq!(xml_path, PathBuf::from("../xml").canonicalize().unwrap());       
/// # }
/// ```
/// # Panics
///
/// Panics if no XML path is provided and no default XML path is not found.
pub fn resolve_xml_path(xml: Option<&str>) -> Result<PathBuf> {
    let mut xml = xml;

    let current_dir: PathBuf = std::env::current_dir()?;

    let default_xml_path = current_dir.join("xml");
    let default_xml_path_alt = current_dir.join("XML");

    // If no XML path is provided, try to find the default XML path.
    if xml.is_none() {
        if default_xml_path.exists() {
            xml = Some(
                default_xml_path
                    .to_str()
                    .expect("default_xml_path is valid UTF-8"),
            );
        }

        if default_xml_path_alt.exists() {
            xml = Some(
                default_xml_path_alt
                    .to_str()
                    .expect("default_xml_path is valid UTF-8"),
            );
        }
    }

    let env_xml_path = std::env::var("LOCKSTEP_XML_PATH");
    if env_xml_path.is_ok() {
        // Override the default, or argument path if the environment variable is set.
        xml = env_xml_path.as_ref().map(|s| s.as_str()).ok();
    }

    // If no XML path is provided and the default XML path is not found, panic.
    if xml.is_none() {
        panic!("No XML path provided and default XML path not found.");
    }

    // Convert, canonicalize and return the XML path.
    let xml = PathBuf::from_str(xml.unwrap())?;
    Ok(xml.canonicalize()?)
}

/// A generic helper to find the file path and interface name of a member.
#[doc(hidden)]
#[macro_export]
macro_rules! find_definition_in_dbus_xml {
    ($xml_path_buf:expr, $member:expr, $iface:expr, $msg_type:expr) => {{
    use zbus_lockstep::MsgType;

    let xml_path_buf: std::path::PathBuf = $xml_path_buf;
    let member: &str = $member;
    let iface: Option<String> = $iface;
    let msg_type: zbus_lockstep::MsgType = $msg_type;

    let mut xml_file_path = None;
    let mut interface_name = None;

    let read_dir = std::fs::read_dir(&xml_path_buf).expect("Failed to read XML directory");

    // Walk the XML files in the directory.
    for entry in read_dir {
        let entry = entry.expect("Failed to read entry");

        // Skip directories and non-XML files.
        if entry.path().is_dir() || entry.path().extension().unwrap() != "xml" {
            continue;
        }

        let entry_path = entry.path().clone();
        let xml_string = std::fs::read_to_string(entry.path()).expect("Failed to read XML file");
        let node = <zbus::xml::Node as std::str::FromStr>::from_str(&xml_string).expect("Failed to parse XML file");

        for interface in node.interfaces() {
            // If called with an `iface` arg, skip he interfaces that do not match.
            if iface.is_some() && interface.name() != iface.clone().unwrap()  {
                continue;
            }

            match msg_type {
                MsgType::Method => {
                    for dbus_item in interface.methods() {
                        if dbus_item.name() == member {
                            if interface_name.is_some() {
                                panic!(
                                    "Multiple interfaces offer the same {:?} member: {}, please specify the interface name.",
                                    msg_type, member
                                );
                            }
                            interface_name = Some(interface.name().to_string());
                            xml_file_path = Some(entry_path.clone());
                            continue;
                        }
                    }
                }
                MsgType::Signal => {
                    for dbus_item in interface.signals() {
                        if dbus_item.name() == member {
                            if interface_name.is_some() {
                                panic!(
                                    "Multiple interfaces offer the same {:?} member: {}, please specify the interface name.",
                                    msg_type, member
                                );
                            }
                            interface_name = Some(interface.name().to_string());
                            xml_file_path = Some(entry_path.clone());
                            continue;
                        }
                    }
                }
                MsgType::Property => {
                    for dbus_item in interface.properties() {
                        if dbus_item.name() == member {
                            if interface_name.is_some() {
                                panic!(
                                    "Multiple interfaces offer the same {:?} member: {}, please specify the interface name.",
                                    msg_type, member
                                );
                            }
                            interface_name = Some(interface.name().to_string());
                            xml_file_path = Some(entry_path.clone());
                            continue;
                        }
                    }
                }
            };
        }
    }

    // If the interface member was not found, return an error.
    if xml_file_path.is_none() {
        panic!("Member not found in XML files.");
    }

    (xml_file_path.unwrap(), interface_name.unwrap())
    }};
}

/// Asserts equality of signatures.
///
/// This macro allows both signatures to be a marshalling round apart.
/// That is, they may differ by one pair of outer parentheses on either side.
///
/// If signatures differ due to marshalling, the difference between marshalled and
/// unmarshalled signatures is one pair of outer parentheses. See
/// [`crate::signatures_are_eq`] for details.
#[macro_export]
macro_rules! assert_eq_signatures {
    ($lhs_sig:expr, $rhs_sig:expr) => {
        assert!(
            zbus_lockstep::signatures_are_eq($lhs_sig, $rhs_sig),
            "Signatures are not equal (Lhs: {}, Rhs: {})",
            $lhs_sig,
            $rhs_sig
        );
    };
}

/// Asserts non-equality of signatures.
///
/// This macro is the inverse of [`assert_eq_signatures`].
/// If signatures differ by more than one pair of outer parentheses -
/// or are otherwise unequal, this macro will pass.
#[macro_export]
macro_rules! assert_ne_signatures {
    ($lhs_sig:expr, $rhs_sig:expr) => {
        assert!(
            !zbus_lockstep::signatures_are_eq($lhs_sig, $rhs_sig),
            "Signatures are equal (Lhs: {}, Rhs: {})",
            $lhs_sig,
            $rhs_sig
        );
    };
}

/// Retrieve the signature of a method's return type.
///
/// This macro will take a method member name and return the signature of the
/// return type.
///
/// Essentially a wrapper around [`zbus_lockstep::get_method_return_type`],
/// but this macro tries to do its job with less arguments.
///
/// It will search in the XML specification of the method for the return type
/// and return the signature of that type.
///
/// If multiple interfaces offer the same method, you will need to specify the
/// interface name as well.
///
/// # Example
///
/// ```rust
/// # use zbus_lockstep::find_definition_in_dbus_xml;
/// # use zbus_lockstep::MsgType;
/// # use zbus_lockstep::resolve_xml_path;
/// # use zbus_lockstep::{method_return_signature, signatures_are_eq};
/// # use zbus::zvariant::Signature;
/// # use zbus_lockstep::assert_eq_signatures;
/// # fn main() {
/// // path to XML files
/// std::env::set_var("LOCKSTEP_XML_PATH", "../xml");
///     
/// let sig = method_return_signature!("RequestName");
/// assert_eq_signatures!(&sig, &Signature::from_str_unchecked("u"));
/// # }
/// ```
#[macro_export]
macro_rules! method_return_signature {
    ($member:expr) => {{
        let member = $member;

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) =
            zbus_lockstep::find_definition_in_dbus_xml!(xml_path, member, None, MsgType::Method);

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_method_return_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};

    ($member:expr, $interface:expr) => {{
        let member = $member;
        let interface = Some($interface.to_string());

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) = zbus_lockstep::find_definition_in_dbus_xml!(
            xml_path,
            member,
            interface,
            MsgType::Method
        );

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_method_return_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};
}

/// Retrieve the signature of a method's arguments.
///
/// Essentially a wrapper around [`zbus_lockstep::get_method_args_type`],
/// but this macro tries to do its job with less arguments.
///
/// This macro will take a method member name and return the signature of the
/// arguments type.
///
/// It will search in the XML specification of the method for the arguments type
/// and return the signature of that type.
///
/// If multiple interfaces offer the same member, you will need to
/// specify the interface name as well.
///
/// This macro can be called with or without the interface name.
#[macro_export]
macro_rules! method_args_signature {
    ($member:expr) => {{
        let member = $member;

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) =
            zbus_lockstep::find_definition_in_dbus_xml!(xml_path, member, None, MsgType::Method);

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_method_args_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};

    ($member:expr, $interface:expr) => {{
        let member = $member;
        let interface = Some($interface.to_string());

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) = zbus_lockstep::find_definition_in_dbus_xml!(
            xml_path,
            member,
            interface,
            MsgType::Method
        );

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_method_args_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};
}

/// Retrieve the signature of a signal's body type.
///
/// Essentially a wrapper around [`zbus_lockstep::get_signal_body_type`],
/// but this macro tries to find it with less arguments.
///
/// This macro will take a signal member name and return the signature of the
/// signal body type.
///
/// If multiple interfaces offer the same member, you will need to
/// specify the interface name as well.
///
/// This macro can be called with or without the interface name.
#[macro_export]
macro_rules! signal_body_type_signature {
    ($member:expr) => {{
        let member = $member;

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) =
            zbus_lockstep::find_definition_in_dbus_xml!(xml_path, member, None, MsgType::Signal);

        let file = std::fs::File::open(file_path).expect("Failed to open file");

        zbus_lockstep::get_signal_body_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};

    ($member:expr, $interface:expr) => {{
        let member = $member;
        let interface = Some($interface.to_string());

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) = zbus_lockstep::find_definition_in_dbus_xml!(
            xml_path,
            member,
            interface,
            MsgType::Signal
        );

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_signal_body_type(file, &interface_name, member, None)
            .expect("Failed to get method arguments type signature")
    }};
}

/// Retrieve the signature of a property's type.
///
/// Essentially a wrapper around [`zbus_lockstep::get_property_type`],
/// but this macro tries to do with less arguments.
///
/// This macro will take a property name and return the signature of the
/// property's type.
///
/// If multiple interfaces offer the same member, you will need to
/// specify the interface name as well.
///
/// This macro can be called with or without the interface name.
#[macro_export]
macro_rules! property_type_signature {
    ($member:expr) => {{
        let member = $member;

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) =
            zbus_lockstep::find_definition_in_dbus_xml!(xml_path, member, None, MsgType::Property);

        let file = std::fs::File::open(file_path).expect("Failed to open file");

        zbus_lockstep::get_property_type(file, &interface_name, member)
            .expect("Failed to get property type signature")
    }};

    ($member:expr, $interface:expr) => {{
        let member = $member;
        let interface = Some($interface.to_string());

        // Looking for default path or path specified by environment variable.
        let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
        let xml_path = zbus_lockstep::resolve_xml_path(None).expect(&format!(
            "Failed to resolve XML path, current dir: {}",
            current_dir.to_str().unwrap()
        ));

        // Find the definition of the method in the XML specification.
        let (file_path, interface_name) = zbus_lockstep::find_definition_in_dbus_xml!(
            xml_path,
            member,
            interface,
            MsgType::Property,
        );

        let file = std::fs::File::open(file_path).expect("Failed to open file");
        zbus_lockstep::get_property_type(file, &interface_name, member)
            .expect("Failed to get property type signature")
    }};
}

#[cfg(test)]
mod test {
    use zbus::zvariant::Signature;

    use crate as zbus_lockstep;
    use crate::{signal_body_type_signature, utils::signatures_are_eq};

    #[test]
    fn test_assert_eq_signatures() {
        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert_eq_signatures!(&sig1, &sig2);

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("a{sv}");

        assert_eq_signatures!(&sig1, &sig2);

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("(a{sv})");

        assert_eq_signatures!(&sig1, &sig2);

        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert_eq_signatures!(&sig1, &sig2);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_signatures_panic() {
        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("ii)(ii");

        assert_eq_signatures!(&sig1, &sig2);
    }

    #[test]
    fn test_assert_ne_signatures() {
        let sig1 = Signature::from_str_unchecked("(ii)");
        let sig2 = Signature::from_str_unchecked("(uu)");

        assert_ne_signatures!(&sig1, &sig2);

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("((a{sv}))");

        assert_ne_signatures!(&sig1, &sig2);

        let sig1 = Signature::from_str_unchecked("(ii(ii))");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert_ne_signatures!(&sig1, &sig2);
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_signatures_panic() {
        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert_ne_signatures!(&sig1, &sig2);
    }

    #[test]
    fn test_signal_body_signature_macro() {
        // path to XML files
        std::env::set_var("LOCKSTEP_XML_PATH", "../xml");

        let sig = zbus_lockstep::signal_body_type_signature!("AddNode");
        assert_eq_signatures!(&sig, &zbus::zvariant::Signature::from_str_unchecked("(so)"));
    }

    #[test]
    fn test_method_args_signature_macro() {
        // path to XML files
        std::env::set_var("LOCKSTEP_XML_PATH", "../xml");

        let sig = zbus_lockstep::method_args_signature!("RequestName");
        assert_eq_signatures!(&sig, &zbus::zvariant::Signature::from_str_unchecked("(su)"));
    }

    #[test]
    fn test_method_return_signature_macro() {
        // path to XML files
        std::env::set_var("LOCKSTEP_XML_PATH", "../xml");

        let sig = zbus_lockstep::method_return_signature!("RequestName");
        assert_eq_signatures!(&sig, &zbus::zvariant::Signature::from_str_unchecked("u"));
    }

    #[test]
    fn test_property_type_signature_macro() {
        // path to XML files
        std::env::set_var("LOCKSTEP_XML_PATH", "../xml");

        let sig = zbus_lockstep::property_type_signature!("Features");
        assert_eq_signatures!(&sig, &zbus::zvariant::Signature::from_str_unchecked("as"));
    }
}

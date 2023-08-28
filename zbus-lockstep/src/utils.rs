use zbus::zvariant::Signature;

/// This is a helper function to check if two signatures are equal,
/// ignoring the parentheses.
///
/// This is useful because the `DBus` marshalling may omit the outer parentheses,
/// or parentheses may be added to the signature by the user when deserializing.
///
/// It is assumed that the signatures are valid.
///
/// # Examples
///
/// ```
/// use zbus::zvariant::Signature;
/// use zbus_lockstep::signatures_are_eq;
///
/// let sig1 = Signature::from_str_unchecked("a{sv}");
/// let sig2 = Signature::from_str_unchecked("(a{sv})");
///
/// assert!(signatures_are_eq(&sig1, &sig2));   
/// ```
///
/// ```
/// use zbus::zvariant::Signature;
/// use zbus_lockstep::signatures_are_eq;
///
/// let sig1 = Signature::from_str_unchecked("(ii)(ii)");
/// let sig2 = Signature::from_str_unchecked("((ii)(ii))");
///
/// assert!(signatures_are_eq(&sig1, &sig2));
/// ```
///
/// # Notes
///
/// There is nothing to prevent you from deserializing "iu" as "(iu)" or "(((iu)))".
/// Also, `DBus` marshalling may omit the outer parentheses of any number of nested
/// 'empty' containers (structs or tuples).
///
/// This function will only consider the common case where the two signatures differ
/// by at most one marshalling step.
///
/// # Safety
///
/// This function assumes that the signatures are valid.
///
/// # Panics
///     
/// This function will panic if the signatures are not valid.
#[must_use]
pub fn signatures_are_eq(lhs: &Signature, rhs: &Signature) -> bool {
    fn strip_parens(bytes: &[u8]) -> &[u8] {
        let [b'(', inner @ .., b')'] = bytes else {
            return bytes;
        };

        // Are these really outer parentheses?
        // Check if the inner part has balanced parentheses.
        // Think of "(i)(i)", which is a valid signature with 'outer parentheses'.
        // But its inner part is not in balance, therefore these are not outer parentheses.

        if inner.iter().fold(0, |count, byte| match byte {
            b'(' => count + 1,
            b')' if count != 0 => count - 1,
            _ => count,
        }) == 0
        {
            inner
        } else {
            bytes
        }
    }

    let lhs_bytes = lhs.as_bytes();
    let rhs_bytes = rhs.as_bytes();

    let lhs_stripped = strip_parens(lhs_bytes);
    let rhs_stripped = strip_parens(rhs_bytes);

    lhs_stripped == rhs_stripped
}

#[cfg(test)]
mod test {
    use zbus::zvariant::Signature;

    use super::signatures_are_eq;

    #[test]
    fn test_signatures_are_eq() {
        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert!(signatures_are_eq(&sig1, &sig2));

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("a{sv}");

        assert!(signatures_are_eq(&sig1, &sig2));

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("(a{sv})");

        assert!(signatures_are_eq(&sig1, &sig2));

        let sig1 = Signature::from_str_unchecked("a{sv}");
        let sig2 = Signature::from_str_unchecked("((a{sv}))");

        assert!(!signatures_are_eq(&sig1, &sig2));

        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("((ii)(ii))");

        assert!(signatures_are_eq(&sig1, &sig2));

        let sig1 = Signature::from_str_unchecked("(ii)(ii)");
        let sig2 = Signature::from_str_unchecked("ii)(ii");

        assert!(!signatures_are_eq(&sig1, &sig2));
    }
}

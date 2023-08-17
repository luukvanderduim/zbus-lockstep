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
///q
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

/// Asserts equality of signatures.
///
/// This macro allows both signatures to be a marshalling round apart.
/// That is, they may differ by one pair of outer parentheses on either side.
///
/// If signatures differ due to marshalling, the difference between marshalled and
/// unmarshalled signatures is one pair of outer parentheses. See [`signatures_are_eq`] for details.
#[macro_export]
macro_rules! assert_eq_signatures {
    ($lhs_sig:expr, $rhs_sig:expr) => {
        assert!(
            signatures_are_eq($lhs_sig, $rhs_sig),
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
            !signatures_are_eq($lhs_sig, $rhs_sig),
            "Signatures are equal (Lhs: {}, Rhs: {})",
            $lhs_sig,
            $rhs_sig
        );
    };
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
}

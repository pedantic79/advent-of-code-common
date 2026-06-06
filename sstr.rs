/// A stack-allocated, fixed-size string wrapper holding exactly `N` bytes.
///
/// `SStr` is useful for representing small ASCII strings (like node names in
/// graphs) without dynamic allocations (`String`). Because it wraps `[u8; N]`,
/// it is `Copy` and implements standard comparison and formatting traits.
///
/// # Safety
/// This structure assumes the inner buffer contains valid ASCII/UTF-8 bytes.
#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct SStr<const N: usize>([u8; N]);

impl<const N: usize> std::str::FromStr for SStr<N> {
    type Err = std::array::TryFromSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().try_into().map(Self)
    }
}

impl<const N: usize> std::fmt::Debug for SStr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> SStr<N> {
    /// Returns the inner buffer as a string slice `&str` of length `N`.
    ///
    /// # Safety
    /// This uses `unsafe` to cast the bytes to a `&str` without validation,
    /// assuming the characters are valid ASCII.
    pub fn as_str(&self) -> &str {
        // SAFETY: In AOC, all characters are ascii
        unsafe { std::str::from_utf8_unchecked(self.0.as_slice()) }
    }

    /// Returns the logical length of the string, excluding any trailing null (`\0`) bytes.
    ///
    /// # Performance
    /// Note that this is an $O(N)$ operation as it scans the underlying array to locate the first null byte.
    pub fn len(&self) -> usize {
        self.0.iter().position(|&b| b == 0).unwrap_or(N)
    }

    /// Returns `true` if the logical string is empty.
    ///
    /// # Performance
    /// This is an $O(1)$ operation as it only checks if the first byte is null.
    pub fn is_empty(&self) -> bool {
        N == 0 || self.0[0] == 0
    }

    /// Returns `true` if the first byte of the string is equal to `p`.
    pub fn starts_with_byte(&self, p: u8) -> bool {
        self.0[0] == p
    }
}

impl<const N: usize> From<[u8; N]> for SStr<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<&str> for SStr<N> {
    /// Converts a `&str` into a `SStr<N>`, padding with null bytes (`\0`) if
    /// the length is less than `N`, and truncating if the length is greater than `N`.
    ///
    /// For a fallible conversion that fails if the string is too long rather than
    /// truncating it, see the [`TryToSStr`] trait.
    fn from(value: &str) -> Self {
        let mut bytes = [0u8; N];
        let len = value.len().min(N);
        bytes[..len].copy_from_slice(&value.as_bytes()[..len]);
        Self(bytes)
    }
}

impl<const N: usize> PartialEq<str> for SStr<N> {
    fn eq(&self, other: &str) -> bool {
        other.len() == N && self.0 == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&str> for SStr<N> {
    fn eq(&self, other: &&str) -> bool {
        other.len() == N && self.0 == other.as_bytes()
    }
}

/// Trait for converting types into a fixed-size [`SStr`] with validation.
pub trait TryToSStr {
    /// Attempts to convert `self` into a [`SStr<N>`].
    ///
    /// Returns `Some(SStr<N>)` if the length of `self` is less than or equal to `N`
    /// (padding with null bytes `\0` if it is too short), or `None` if the length
    /// is greater than `N`.
    fn try_to_sstr<const N: usize>(&self) -> Option<SStr<N>>;
}

impl TryToSStr for str {
    fn try_to_sstr<const N: usize>(&self) -> Option<SStr<N>> {
        if self.len() <= N {
            let mut bytes = [0u8; N];
            bytes[..self.len()].copy_from_slice(self.as_bytes());
            Some(SStr::from(bytes))
        } else {
            None
        }
    }
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_sstr_creation_and_methods() {
        let raw = [b'a', b'b', b'c'];
        let sstr: SStr<3> = SStr::from(raw);
        assert_eq!(sstr.as_str(), "abc");
        assert!(sstr.starts_with_byte(b'a'));
        assert!(!sstr.starts_with_byte(b'b'));

        let from_str = SStr::<3>::from("abc");
        assert_eq!(from_str.as_str(), "abc");

        let parsed = SStr::<3>::from_str("abc");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().as_str(), "abc");

        let parsed_err = SStr::<3>::from_str("abcd");
        assert!(parsed_err.is_err());

        let from_str_exact = SStr::<3>::from("abc");
        assert_eq!(from_str_exact.as_str(), "abc");

        let from_str_short = SStr::<3>::from("ab");
        assert_eq!(from_str_short.as_str(), "ab\0");

        let from_str_long = SStr::<3>::from("abcd");
        assert_eq!(from_str_long.as_str(), "abc");

        let try_exact = "abc".try_to_sstr::<3>();
        assert!(try_exact.is_some());
        assert_eq!(try_exact.unwrap().as_str(), "abc");

        let try_short = "ab".try_to_sstr::<3>();
        assert!(try_short.is_some());
        assert_eq!(try_short.unwrap().as_str(), "ab\0");

        let try_long = "abcd".try_to_sstr::<3>();
        assert!(try_long.is_none());

        assert_eq!(SStr::<3>::from("abc").len(), 3);
        assert!(!SStr::<3>::from("abc").is_empty());

        assert_eq!(SStr::<3>::from("ab").len(), 2);
        assert!(!SStr::<3>::from("ab").is_empty());

        assert_eq!(SStr::<3>::from("").len(), 0);
        assert!(SStr::<3>::from("").is_empty());
    }

    #[test]
    fn test_sstr_equality() {
        let sstr: SStr<3> = SStr::from("abc");
        assert_eq!(sstr, "abc");
        assert_eq!(sstr, *"abc");
        assert_ne!(sstr, "abcd");
        assert_ne!(sstr, "ab");
    }

    #[test]
    fn test_sstr_formatting() {
        let sstr: SStr<3> = SStr::from("abc");
        assert_eq!(format!("{:?}", sstr), "abc");
    }
}

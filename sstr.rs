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
    pub fn as_str(&self) -> &str {
        // SAFETY: In AOC, all characters are ascii
        unsafe { std::str::from_utf8_unchecked(self.0.as_slice()) }
    }

    pub fn starts_with(&self, p: u8) -> bool {
        self.0[0] == p
    }
}

impl<const N: usize> From<[u8; N]> for SStr<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<&str> for SStr<N> {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().try_into().unwrap())
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
        assert!(sstr.starts_with(b'a'));
        assert!(!sstr.starts_with(b'b'));

        let from_str = SStr::<3>::from("abc");
        assert_eq!(from_str.as_str(), "abc");

        let parsed = SStr::<3>::from_str("abc");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().as_str(), "abc");

        let parsed_err = SStr::<3>::from_str("abcd");
        assert!(parsed_err.is_err());
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

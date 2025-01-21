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

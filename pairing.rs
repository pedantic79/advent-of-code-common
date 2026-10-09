// Branchless codegen contract: these unchecked primitive conversions must stay
// branchless on Apple Silicon Macs (aarch64-apple-darwin) in optimized builds.
// Verified with rustc 1.100.0-beta.4, opt-level=3, panic=abort, and both generic
// and native CPUs. Recheck assembly when changing the compiler or build settings.

/// Pair unsigned coordinates that fit usize without overflowing the arithmetic.
pub fn pair<T>(x: T, y: T) -> usize
where
    T: num::Unsigned + num::PrimInt,
{
    // SAFETY: The caller must ensure both coordinates fit usize.
    let to_usize = |o: T| unsafe { o.to_usize().unwrap_unchecked() };
    let x1 = to_usize(x);
    let y1 = to_usize(y);

    let sum = x1 + y1;
    sum * (sum + 1) / 2 + y1
}

#[inline]
fn unpair_internal(z: usize) -> (usize, usize) {
    let w = (((8 * z + 1) as f64).sqrt() as usize - 1) / 2;
    let t = w * (w + 1) / 2;
    let y = z - t;
    let x = w - y;

    (x, y)
}

/// Decode a pairing whose coordinates fit T and whose 8 * z + 1 fits usize.
#[inline]
pub fn unpair<T>(z: usize) -> (T, T)
where
    T: num::Unsigned + num::PrimInt,
{
    // SAFETY: The caller must ensure both decoded coordinates fit T.
    let from_usize = |n| unsafe { T::from(n).unwrap_unchecked() };
    let (x, y) = unpair_internal(z);
    (from_usize(x), from_usize(y))
}

/// pair2d_a is a cantor pairing function extended to signed numbers
/// It does this by storing the sign bit in the right most spot. Shifting the positive value to the left one spot
/// so -8 becomes 0b1001
///
/// Then it performs a cantor pairing on the positive numbers
/// The sign-encoded magnitudes must fit both T and i32, and the pairing
/// arithmetic must fit usize.
#[inline]
pub fn pair2d_a<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt,
{
    // SAFETY: The caller must ensure the sign-encoded magnitudes fit i32.
    let to_usize = |o: T| (unsafe { o.to_i32().unwrap_unchecked() }) as usize;

    let x1 = to_usize((x.abs() << 1) | T::from(x.is_negative() as u8).unwrap());
    let y1 = to_usize((y.abs() << 1) | T::from(y.is_negative() as u8).unwrap());

    pair(x1, y1)
}

/// Decode a pairing whose absolute coordinate magnitudes fit T.
/// The arithmetic in unpair_internal must also fit usize.
#[inline]
pub fn unpair2d_a<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt,
{
    let from_usize = |n: usize| {
        let sign = if n & 1 == 1 { -T::one() } else { T::one() };
        // SAFETY: The caller must ensure the decoded magnitudes fit T.
        // Shifting usize right by one guarantees the intermediate fits isize.
        let value = unsafe { T::from((n >> 1) as isize).unwrap_unchecked() };
        sign * value
    };
    let (x, y) = unpair_internal(z);

    (from_usize(x), from_usize(y))
}

/// pair2d_b is a cantor pairing function extended to signed numbers
/// It does this by retrieving the sign bits (xsign or ysign)
/// It then maps the number to positive by XOR with -1 (all 1's in binary) if the number is negative
/// so -8 becomes 7, or -235 becomes 234
///
/// Then it performs a cantor pairing on the positive numbers
/// It then shifts the number over by 2 bits to the left and stores the signed bits in the remaining spot
/// The XOR-mapped magnitudes must fit i32, and the pairing arithmetic must
/// fit usize.
#[inline]
pub fn pair2d_b<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt + num::FromPrimitive,
{
    // SAFETY: The caller must ensure the XOR-mapped magnitudes fit i32;
    // the sign flags are always zero or one and therefore also fit.
    let to_usize = |o: T| (unsafe { o.to_i32().unwrap_unchecked() }) as usize;
    let neg = T::from_i8(-1).unwrap();

    let xsign = T::from_u8(x.is_negative() as u8).unwrap();
    let ysign = T::from_u8(y.is_negative() as u8).unwrap();

    let x1 = to_usize(x ^ (xsign * neg));
    let y1 = to_usize(y ^ (ysign * neg));
    let sum = x1 + y1;
    let tri = sum * (sum + 1) / 2 + y1;

    (to_usize(xsign) | (to_usize(ysign) << 1)) + tri * 4
}

/// Decode a pairing whose XOR-mapped magnitudes fit both i32 and T.
/// The arithmetic in unpair_internal must also fit usize.
pub fn unpair2d_b<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt + num::FromPrimitive,
{
    // SAFETY: The caller must ensure the decoded magnitudes fit i32 and T;
    // the sign flags are always zero or one and therefore also fit.
    let from_usize = |o: usize| unsafe { T::from_i32(o as i32).unwrap_unchecked() };
    let neg = T::from_i8(-1).unwrap();

    let sign_bits = z % 4;
    let z = z >> 2;

    let (x1, y1) = unpair_internal(z);

    let xsign = sign_bits & 1;
    let ysign = (sign_bits >> 1) & 1;

    let x = from_usize(x1) ^ (from_usize(xsign) * neg);
    let y = from_usize(y1) ^ (from_usize(ysign) * neg);

    (x, y)
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsigned_roundtrips_across_primitive_widths() {
        macro_rules! check {
            ($($ty:ty),+) => {
                $(
                    for x in 0..=10 as $ty {
                        for y in 0..=10 as $ty {
                            assert_eq!(unpair::<$ty>(pair(x, y)), (x, y));
                        }
                    }
                )+
            };
        }

        check!(u8, u16, u32, u64, usize, u128);
    }

    #[test]
    fn signed_roundtrips_across_primitive_widths() {
        macro_rules! check {
            ($($ty:ty),+) => {
                $(
                    for x in -10..=10 as $ty {
                        for y in -10..=10 as $ty {
                            assert_eq!(unpair2d_a::<$ty>(pair2d_a(x, y)), (x, y));
                            assert_eq!(unpair2d_b::<$ty>(pair2d_b(x, y)), (x, y));
                        }
                    }
                )+
            };
        }

        check!(i8, i16, i32, i64, isize, i128);
    }

    #[test]
    fn signed_a_decodes_narrow_magnitudes_at_their_limits() {
        macro_rules! check {
            ($ty:ty, $limit:expr) => {
                let limit: $ty = $limit;
                for x in [-limit, -1, 0, 1, limit] {
                    for y in [-limit, -1, 0, 1, limit] {
                        // The encoder needs room for twice the magnitude.
                        let encoded = pair2d_a(i64::from(x), i64::from(y));
                        assert_eq!(unpair2d_a::<$ty>(encoded), (x, y));
                    }
                }
            };
        }

        check!(i8, i8::MAX);
        check!(i16, i16::MAX);
    }

    #[test]
    fn rountrip_a() {
        for i in -10..10 {
            for j in -10..10 {
                let a = pair2d_a(i, j);
                let p = unpair2d_a(a);

                assert_eq!((i, j), p)
            }
        }
    }

    #[test]
    fn rountrip_b() {
        for i in -10..10 {
            for j in -10..10 {
                let a = pair2d_b(i, j);
                let p = unpair2d_b(a);

                assert_eq!((i, j), p)
            }
        }
    }
}

//! Branchless Cantor pairing for primitive integers.
//!
//! Numeric casts truncate like Rust's `as`, and arithmetic wraps.
//! Exact roundtrips require coordinates and results to fit their integer types,
//! pairing arithmetic not to overflow, and `8 * z + 1` to fit `usize` on decode.

use num::traits::AsPrimitive;

/// Pair unsigned coordinates, casting them to usize before pairing.
///
/// An exact result requires both coordinates and every intermediate in
/// `(x + y) * (x + y + 1) / 2 + y` to fit `usize`.
/// See [`unpair`] for the additional bounds needed for a roundtrip.
///
/// Outside these bounds, casts truncate and arithmetic wraps, which can lose
/// information.
#[inline]
pub fn pair<T>(x: T, y: T) -> usize
where
    T: num::Unsigned + num::PrimInt + AsPrimitive<usize>,
{
    let x = x.as_();
    let y = y.as_();
    let sum = x.wrapping_add(y);

    (sum.wrapping_mul(sum.wrapping_add(1)) / 2).wrapping_add(y)
}

#[inline]
fn unpair_internal(z: usize) -> (usize, usize) {
    let discriminant = z.wrapping_mul(8).wrapping_add(1);
    let estimate = ((discriminant as f64).sqrt() as usize).wrapping_sub(1) / 2;

    // Floating-point rounding can put the estimate on either side of a
    // triangular boundary. Correct by at most one without conditional control flow.
    let triangular = |w: usize| w.wrapping_mul(w.wrapping_add(1)) / 2;
    let w = estimate.wrapping_sub((triangular(estimate) > z) as usize);
    let w = w.wrapping_add((triangular(w.wrapping_add(1)) <= z) as usize);
    let y = z.wrapping_sub(triangular(w));

    (w.wrapping_sub(y), y)
}

/// Decode unsigned coordinates.
///
/// Exact decoding requires `8 * z + 1` to fit `usize` and both decoded
/// coordinates to fit `T`.
///
/// Outside these bounds, arithmetic wraps and output casts truncate, so the
/// result may be incorrect.
#[inline]
pub fn unpair<T>(z: usize) -> (T, T)
where
    T: num::Unsigned + num::PrimInt + 'static,
    usize: AsPrimitive<T>,
{
    let (x, y) = unpair_internal(z);

    (x.as_(), y.as_())
}

/// Pair signed coordinates using twice the magnitude plus a low sign bit.
///
/// An exact result requires both coordinates to fit `isize`, their encoded
/// magnitudes `2 * abs(value) + is_negative` to fit `usize`, and the arithmetic
/// in [`pair`] to fit `usize`. See [`unpair2d_a`] for the decoding bounds.
///
/// Outside these bounds, casts truncate and arithmetic wraps, which can lose
/// information.
#[inline]
pub fn pair2d_a<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt + AsPrimitive<isize>,
{
    let encode = |value: isize| value.unsigned_abs().wrapping_shl(1) | ((value < 0) as usize);

    pair(encode(x.as_()), encode(y.as_()))
}

/// Decode the magnitude/sign encoding produced by [`pair2d_a`].
///
/// Exact decoding requires `8 * z + 1` to fit `usize` and both signed decoded
/// coordinates to fit `T`.
///
/// Outside these bounds, arithmetic wraps and output casts truncate, so the
/// result may be incorrect.
#[inline]
pub fn unpair2d_a<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt + 'static,
    isize: AsPrimitive<T>,
{
    let decode = |n: usize| {
        let sign_mask = -((n & 1) as isize);
        (((n >> 1) as isize ^ sign_mask).wrapping_sub(sign_mask)).as_()
    };
    let (x, y) = unpair_internal(z);

    (decode(x), decode(y))
}

/// Pair signed coordinates using XOR magnitudes and two low sign bits.
///
/// An exact result requires both coordinates to fit `isize`, the arithmetic in
/// [`pair`] for the XOR-mapped magnitudes to fit `usize`, and the final value
/// `4 * paired_magnitudes + sign_bits` to fit `usize`.
/// See [`unpair2d_b`] for the decoding bounds.
///
/// Outside these bounds, casts truncate and arithmetic wraps, which can lose
/// information.
#[inline]
pub fn pair2d_b<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt + AsPrimitive<isize>,
{
    let x: isize = x.as_();
    let y: isize = y.as_();
    let xsign = (x < 0) as usize;
    let ysign = (y < 0) as usize;
    let x1 = (x as usize) ^ 0_usize.wrapping_sub(xsign);
    let y1 = (y as usize) ^ 0_usize.wrapping_sub(ysign);

    pair(x1, y1).wrapping_mul(4) | xsign | (ysign << 1)
}

/// Decode the XOR/sign encoding produced by [`pair2d_b`].
///
/// Exact decoding requires `8 * (z >> 2) + 1` to fit `usize` and both signed
/// decoded coordinates to fit `T`.
///
/// Outside these bounds, arithmetic wraps and output casts truncate, so the
/// result may be incorrect.
#[inline]
pub fn unpair2d_b<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt + 'static,
    isize: AsPrimitive<T>,
{
    let (x, y) = unpair_internal(z >> 2);
    let xsign = (z & 1) as isize;
    let ysign = ((z >> 1) & 1) as isize;
    let x = (x as isize) ^ -xsign;
    let y = (y as isize) ^ -ysign;

    (x.as_(), y.as_())
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
    fn decodes_both_sides_of_large_triangular_boundaries() {
        let largest_sum = (1_usize << (usize::BITS / 2 - 1)) - 1;

        for sum in [
            1,
            2,
            127,
            255,
            largest_sum.min(65_535),
            largest_sum - 1,
            largest_sum,
        ] {
            let triangular = pair(sum, 0);

            assert_eq!(unpair::<usize>(triangular - 1), (0, sum - 1));
            assert_eq!(unpair::<usize>(triangular), (sum, 0));
            assert_eq!(unpair::<usize>(triangular + 1), (sum - 1, 1));
        }
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

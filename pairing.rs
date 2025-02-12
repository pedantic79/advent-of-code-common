pub fn pair<T>(x: T, y: T) -> usize
where
    T: num::Unsigned + num::PrimInt,
{
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

#[inline]
pub fn unpair<T>(z: usize) -> (T, T)
where
    T: num::Unsigned + num::PrimInt,
{
    let from_usize = |n| unsafe { T::from(n).unwrap_unchecked() };
    let (x, y) = unpair_internal(z);
    (from_usize(x), from_usize(y))
}

/// pair2d_a is a cantor pairing function extended to signed numbers
/// It does this by storing the sign bit in the right most spot. Shifting the positive value to the left one spot
/// so -8 becomes 0b1001
///
/// Then it performs a cantor pairing on the positive numbers
#[inline]
pub fn pair2d_a<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt,
{
    // make sure the compiler optimises this conversion for types smaller than usize
    let to_usize = |o: T| (unsafe { o.to_i32().unwrap_unchecked() }) as usize;

    let x1 = to_usize((x.abs() << 1) | T::from(x.is_negative() as u8).unwrap());
    let y1 = to_usize((y.abs() << 1) | T::from(y.is_negative() as u8).unwrap());

    pair(x1, y1)
}

#[inline]
pub fn unpair2d_a<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt,
{
    let from_usize = |n: usize| {
        let sign = if n & 1 == 1 { -T::one() } else { T::one() };
        let value = T::from((n >> 1) as isize).unwrap();
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
#[inline]
pub fn pair2d_b<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt + num::FromPrimitive,
{
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

pub fn unpair2d_b<T>(z: usize) -> (T, T)
where
    T: num::Signed + num::PrimInt + num::FromPrimitive,
{
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

#[cfg(test)]
mod test {
    use super::*;

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

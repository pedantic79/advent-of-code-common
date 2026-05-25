use super::extensions::AddIsize;
use arrayvec::ArrayVec;
use std::{fmt::Debug, ops::Mul};

pub use super::parse::*;

pub mod rev;

pub trait MyInteger: num::Integer + Clone + for<'a> Mul<&'a Self, Output = Self> {}

impl<T> MyInteger for T where T: num::Integer + Clone + for<'a> Mul<&'a T, Output = T> {}

// Based on the C++ algorithm here: https://stackoverflow.com/a/53604277/7263440
#[inline]
pub fn mod_inv<U>(mut a: U, mut m: U) -> U
where
    U: MyInteger,
{
    if m <= U::one() {
        return U::zero();
    }

    let m0 = m.clone();
    let mut x0 = (U::zero(), false);
    let mut x1 = (U::one(), false);

    while a > U::one() {
        if m == U::zero() {
            return U::zero();
        }

        let (q, temp) = a.div_rem(&m);
        a = m;
        m = temp;

        let q = q.mul(&x0.0);

        x1 = if x0.1 != x1.1 {
            (x1.0 + q, x1.1)
        } else if x1.0 > q {
            (x1.0 - q, x1.1)
        } else {
            (q - x1.0, !x0.1)
        };

        std::mem::swap(&mut x0, &mut x1);
    }

    if x1.1 { m0 - x1.0 } else { x1.0 }
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: MyInteger,
{
    if modulus == T::one() {
        return T::zero();
    }

    let mut result = T::one();
    base = base % modulus.clone();
    while exp > T::zero() {
        if exp.is_odd() {
            result = result * base.clone() % modulus.clone();
        }

        exp = exp / (T::one() + T::one());
        base = base.clone() * base % modulus.clone()
    }
    result
}

pub fn baby_step_giant_step<I>(modulo: I, base: I, target: I) -> Option<I>
where
    I: MyInteger + num::integer::Roots + num::ToPrimitive + std::hash::Hash,
{
    let m = num::integer::sqrt(modulo.clone());

    let precomp = num::range(I::zero(), m.clone())
        .map(|j| (mod_pow(base.clone(), j.clone(), modulo.clone()), j))
        .collect::<std::collections::HashMap<_, _>>();

    let invgenerator = mod_inv(mod_pow(base, m.clone(), modulo.clone()), modulo.clone());
    let mut value = target;

    for i in num::range(I::zero(), m.clone()) {
        if let Some(v) = precomp.get(&value) {
            return Some(i * m + v.clone());
        }

        value = value * invgenerator.clone() % modulo.clone();
    }

    None
}

pub fn chinese_remainder_theorem<T, I>(inputs: I) -> T
where
    T: MyInteger,
    I: Iterator<Item = (T, T)> + Clone,
{
    let mut product = T::one();

    for n in inputs.clone() {
        product = product * n.1;
    }

    let mut sum = T::zero();
    for (x, m) in inputs {
        let a = product.clone() / m.clone();
        let y = mod_inv(a.clone(), m.clone());

        sum = sum + x * a * y;
    }

    sum % product
}

pub fn build_array<T, I, const N: usize>(iter: I) -> [T; N]
where
    T: Debug,
    I: IntoIterator<Item = T>,
{
    iter.into_iter()
        .collect::<ArrayVec<T, N>>()
        .into_inner()
        .unwrap()
}

pub fn neighbors_arbitray(
    diffs: &[(isize, isize)],
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    diffs.iter().filter_map(move |&(y, x)| {
        let r_new = r.checked_add_isize_clamp(y, r_max)?;
        let c_new = c.checked_add_isize_clamp(x, c_max)?;

        Some((r_new, c_new))
    })
}

pub fn neighbors(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(&[(-1, 0), (0, -1), (0, 1), (1, 0)], r, c, r_max, c_max)
}

pub fn neighbors_and_self(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[(0, 0), (-1, 0), (0, -1), (0, 1), (1, 0)],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn neighbors_diag(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn neighbors_diag_and_self(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[
            (0, 0),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn slice_get_mut_twice<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    assert_ne!(index0, index1);
    assert!(index0 < slice.len());
    assert!(index1 < slice.len());

    // SAFETY: guarantee that the indices are never the same. So it is safe to
    // have two mutable references into the Vec. We'll double check that the
    // indices are within the bounds.
    unsafe {
        let ptr = slice.as_mut_ptr();
        let one = &mut *ptr.add(index0);
        let two = &mut *ptr.add(index1);
        (one, two)
    }
}

pub fn calculate_area_perimeter<T>(points: impl Iterator<Item = (T, T)>) -> (T, T)
where
    T: num::PrimInt + num::Signed,
{
    let (a, p, _) = points.fold(
        (T::zero(), T::zero(), (T::zero(), T::zero())),
        |(area, perimeter, prev), curr| {
            (
                area + (prev.0 * curr.1 - prev.1 * curr.0),
                perimeter + (prev.0 - curr.0).abs() + (prev.1 - curr.1).abs(),
                curr,
            )
        },
    );

    (a, p)
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_arithmetic() {
        // mod_inv
        // Inverse of 3 modulo 11 is 4, because 3 * 4 = 12 = 1 mod 11
        assert_eq!(mod_inv(3, 11), 4);
        assert_eq!(mod_inv(2, 5), 3);

        // mod_pow
        // 2^10 mod 1000 = 1024 mod 1000 = 24
        assert_eq!(mod_pow(2, 10, 1000), 24);
        // 5^3 mod 7 = 125 mod 7 = 6 (since 17 * 7 = 119)
        assert_eq!(mod_pow(5, 3, 7), 6);
    }

    #[test]
    fn test_discrete_log_and_crt() {
        // baby_step_giant_step
        // Solve: 2^x = 8 (mod 11). x should be 3
        assert_eq!(baby_step_giant_step(11, 2, 8), Some(3));
        // Solve: 2^x = 5 (mod 11). Let's check powers of 2 mod 11:
        // 2^0=1, 2^1=2, 2^2=4, 2^3=8, 2^4=5. x should be 4
        assert_eq!(baby_step_giant_step(11, 2, 5), Some(4));
        // Unsolvable
        assert_eq!(baby_step_giant_step(11, 2, 0), None);

        // chinese_remainder_theorem
        // x = 2 mod 3
        // x = 3 mod 5
        // x = 2 mod 7
        // Answer is 23
        let inputs = vec![(2, 3), (3, 5), (2, 7)];
        assert_eq!(chinese_remainder_theorem(inputs.into_iter()), 23);
    }

    #[test]
    fn test_build_array() {
        let iter = vec![1, 2, 3].into_iter();
        let arr: [i32; 3] = build_array(iter);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_neighbors() {
        // 3x3 grid, neighbors of (1, 1) should be (0, 1), (1, 0), (1, 2), (2, 1)
        let mut normal: Vec<(usize, usize)> = neighbors(1, 1, 3, 3).collect();
        normal.sort();
        let mut expected = vec![(0, 1), (1, 0), (1, 2), (2, 1)];
        expected.sort();
        assert_eq!(normal, expected);

        // neighbors and self
        let mut and_self: Vec<(usize, usize)> = neighbors_and_self(1, 1, 3, 3).collect();
        and_self.sort();
        let mut expected_and_self = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
        expected_and_self.sort();
        assert_eq!(and_self, expected_and_self);

        // neighbors diag
        let mut diag: Vec<(usize, usize)> = neighbors_diag(1, 1, 3, 3).collect();
        diag.sort();

        #[rustfmt::skip]
        let mut expected_diag = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2),
        ];
        expected_diag.sort();
        assert_eq!(diag, expected_diag);
    }

    #[test]
    fn test_slice_get_mut_twice() {
        let mut data = vec![100, 200, 300];
        let (x, y) = slice_get_mut_twice(&mut data, 0, 2);
        assert_eq!(*x, 100);
        assert_eq!(*y, 300);
        *x = 105;
        *y = 305;
        assert_eq!(data, vec![105, 200, 305]);
    }

    #[test]
    #[should_panic]
    fn test_slice_get_mut_twice_panic() {
        let mut data = vec![100, 200, 300];
        let _ = slice_get_mut_twice(&mut data, 1, 1);
    }

    #[test]
    fn test_calculate_area_perimeter() {
        // Let's calculate area and perimeter of a 2x2 square
        // Points: (0,0) -> (0,2) -> (2,2) -> (2,0) -> (0,0) (closed loop)
        let points = vec![(0i32, 0i32), (0, 2), (2, 2), (2, 0), (0, 0)];
        let (area, perimeter) = calculate_area_perimeter(points.into_iter());
        // Green's theorem / Shoelace double area:
        // Sum(x_i * y_{i+1} - y_i * x_{i+1})
        // (0*2 - 0*0) + (0*2 - 2*2) + (2*0 - 2*2) + (2*0 - 0*0)
        // 0 - 4 - 4 + 0 = -8. The absolute double area is 8, so actual area is 4 (or -4/8 depending on orientation/double area representation).
        // Let's check:
        // area = 0*2 - 0*0 + 0*2 - 2*2 + 2*0 - 2*2 + 2*0 - 0*0 = -8.
        // perimeter = |0-0| + |0-2| + |0-2| + |2-2| + |2-2| + |2-0| + |2-0| + |0-0| = 2 + 2 + 2 + 2 = 8.
        assert_eq!(area.abs(), 8); // Double area is returned
        assert_eq!(perimeter, 8);
    }
}

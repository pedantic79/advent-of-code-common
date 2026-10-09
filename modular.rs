use num::{PrimInt, Signed, traits::Euclid};

use super::utils::{MyInteger, mod_inv};

/// An affine map `x -> multiplier * x + offset` modulo a positive integer.
///
/// Coefficients and input positions must be normalized to `0..modulus`.
/// Choose a type wide enough for `(modulus - 1)^2 + modulus - 1` and the
/// intermediate extended-Euclidean arithmetic; multiplication is not widened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Affine<T> {
    pub multiplier: T,
    pub offset: T,
}

impl<T: PrimInt + Signed + Euclid + MyInteger> Affine<T> {
    pub fn identity() -> Self {
        Self {
            multiplier: T::one(),
            offset: T::zero(),
        }
    }

    #[inline]
    pub fn apply(self, position: T, modulus: T) -> T {
        (self.multiplier * position + self.offset).rem_euclid(&modulus)
    }

    /// Compose this map with `next`, applying `next` last.
    #[inline]
    pub fn then(self, next: Self, modulus: T) -> Self {
        Self {
            multiplier: (next.multiplier * self.multiplier).rem_euclid(&modulus),
            offset: next.apply(self.offset, modulus),
        }
    }

    /// Exponentiation by squaring, including translations and zero repetitions.
    #[inline]
    pub fn repeat(mut self, mut repetitions: u64, modulus: T) -> Self {
        let mut result = Self::identity();

        while repetitions != 0 {
            if repetitions & 1 != 0 {
                result = result.then(self, modulus);
            }
            self = self.then(self, modulus);
            repetitions >>= 1;
        }

        result
    }

    /// Return a preimage when the multiplier is known to be coprime to the modulus.
    /// This avoids the invertibility checks in `preimage` for established permutations.
    #[inline]
    pub fn preimage_coprime(self, position: T, modulus: T) -> T {
        ((position - self.offset) * mod_inv(self.multiplier, modulus)).rem_euclid(&modulus)
    }

    /// Return the unique preimage, or `None` when the map is not invertible.
    #[inline]
    pub fn preimage(self, position: T, modulus: T) -> Option<T> {
        if self.multiplier == T::zero() {
            return (modulus == T::one()).then(T::zero);
        }

        // The shared inverse computes only the needed coefficient and stops
        // at gcd == 1; the crate's full extended_gcd measured slower here.
        let inverse = mod_inv(self.multiplier, modulus);
        (inverse != T::zero() || modulus == T::one())
            .then(|| ((position - self.offset) * inverse).rem_euclid(&modulus))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_and_repetition_match_literal_application() {
        for modulus in 2_i64..=17 {
            for multiplier in 0..modulus {
                let map = Affine {
                    multiplier,
                    offset: 1,
                };
                for position in 0..modulus {
                    let mut literal = position;
                    for repetitions in 0..=20 {
                        assert_eq!(
                            map.repeat(repetitions, modulus).apply(position, modulus),
                            literal
                        );
                        literal = map.apply(literal, modulus);
                    }
                }
            }
        }
    }

    #[test]
    fn inverse_detects_noninvertible_maps_and_supports_other_integer_types() {
        let map = Affine {
            multiplier: 3_i128,
            offset: 4,
        };
        for position in 0..11 {
            assert_eq!(map.preimage(map.apply(position, 11), 11), Some(position));
        }
        assert_eq!(
            Affine {
                multiplier: 2_i32,
                offset: 1
            }
            .preimage(3, 10),
            None
        );
    }

    #[test]
    fn inverses_match_every_small_modular_map() {
        for modulus in 2_i64..=23 {
            for multiplier in 0..modulus {
                let map = Affine {
                    multiplier,
                    offset: modulus - 1,
                };
                for position in 0..modulus {
                    let expected =
                        (num::integer::gcd(multiplier, modulus) == 1).then_some(position);
                    assert_eq!(
                        map.preimage(map.apply(position, modulus), modulus),
                        expected
                    );
                    if expected.is_some() {
                        assert_eq!(
                            map.preimage_coprime(map.apply(position, modulus), modulus),
                            position
                        );
                    }
                }
            }
        }
        assert_eq!(
            Affine {
                multiplier: 0_i32,
                offset: 0
            }
            .preimage(0, 1),
            Some(0)
        );
    }
}

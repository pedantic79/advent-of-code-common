//! Safety regressions, run both natively and with `cargo +nightly miri test
//! --lib --features common_test common::miri_tests`.

use super::{GetMutTwice, pairing::*, parse::*, sstr::SStr, utils::slice_get_mut_twice};

#[test]
fn unsigned_pairing_defines_truncation_and_overflow_for_every_width() {
    macro_rules! check {
        ($($ty:ty),+) => {
            $(
                for x in [0, 1, <$ty>::MAX] {
                    for y in [0, 1, <$ty>::MAX] {
                        assert_eq!(pair(x, y), pair(x as usize, y as usize));
                    }
                }

                for z in [0, 1, usize::MAX / 8, usize::MAX / 8 + 1, usize::MAX] {
                    let (x, y) = unpair::<usize>(z);

                    assert_eq!(unpair::<$ty>(z), (x as $ty, y as $ty));
                }
            )+
        };
    }

    check!(u8, u16, u32, u64, usize, u128);
}

#[test]
fn signed_pairing_defines_truncation_and_overflow_for_every_width() {
    macro_rules! check {
        ($($ty:ty),+) => {
            $(
                for x in [<$ty>::MIN, -1, 0, 1, <$ty>::MAX] {
                    for y in [<$ty>::MIN, -1, 0, 1, <$ty>::MAX] {
                        assert_eq!(pair2d_a(x, y), pair2d_a(x as isize, y as isize));
                        assert_eq!(pair2d_b(x, y), pair2d_b(x as isize, y as isize));
                    }
                }

                for z in [0, 1, usize::MAX / 8, usize::MAX / 8 + 1, usize::MAX] {
                    let (ax, ay) = unpair2d_a::<isize>(z);
                    let (bx, by) = unpair2d_b::<isize>(z);

                    assert_eq!(unpair2d_a::<$ty>(z), (ax as $ty, ay as $ty));
                    assert_eq!(unpair2d_b::<$ty>(z), (bx as $ty, by as $ty));
                }
            )+
        };
    }

    check!(i8, i16, i32, i64, isize, i128);
}

#[test]
fn narrow_decoders_truncate_without_undefined_behavior() {
    let unsigned = pair(256_usize, 257);
    let signed_a = pair2d_a(-129_isize, 128);
    let signed_b = pair2d_b(-129_isize, 128);

    assert_eq!(unpair::<u8>(unsigned), (0, 1));
    assert_eq!(unpair2d_a::<i8>(signed_a), (127, -128));
    assert_eq!(unpair2d_b::<i8>(signed_b), (127, -128));
}

#[test]
fn pairing_decoders_accept_narrow_boundaries() {
    for x in [0_u8, 1, u8::MAX] {
        for y in [0_u8, 1, u8::MAX] {
            assert_eq!(unpair::<u8>(pair(x, y)), (x, y));
        }
    }

    for x in [i8::MIN, -1, 0, 1, i8::MAX] {
        for y in [i8::MIN, -1, 0, 1, i8::MAX] {
            assert_eq!(unpair2d_b::<i8>(pair2d_b(x, y)), (x, y));
        }
    }
}

#[test]
fn unicode_patterns_preserve_utf8_boundaries() {
    let inputs = ["", "é", "🦀é🦀", "é🦀é", "a\0é🦀z"];
    let delimiters = ["", "é", "🦀", "é🦀", "\0", "missing"];

    for input in inputs {
        for delimiter in delimiters {
            assert_eq!(
                parse_split_once::<String, _>(input, delimiter),
                input
                    .split_once(delimiter)
                    .map(|(left, right)| (left.to_owned(), right.to_owned()))
            );
            assert_eq!(
                parse_split::<String, _>(input, delimiter),
                input.split(delimiter).collect::<Vec<_>>()
            );
        }

        assert_eq!(
            parse_split::<String, _>(input, '🦀'),
            input.split('🦀').collect::<Vec<_>>()
        );
        assert_eq!(
            parse_split::<String, _>(input, |c: char| !c.is_ascii()),
            input.split(|c: char| !c.is_ascii()).collect::<Vec<_>>()
        );
    }
}

// Pattern is a safe trait. Its searcher may use a different haystack while
// satisfying every safety requirement of Searcher for that haystack.
struct RedirectedPattern;

impl<'a> stable_pattern::Pattern<'a> for RedirectedPattern {
    type Searcher = <char as stable_pattern::Pattern<'a>>::Searcher;

    fn into_searcher(self, _haystack: &'a str) -> Self::Searcher {
        stable_pattern::Pattern::into_searcher(',', "é,🦀")
    }
}

#[test]
fn custom_pattern_uses_its_searchers_haystack() {
    assert_eq!(
        parse_split_once::<String, _>("x", RedirectedPattern),
        Some(("é".to_owned(), "🦀".to_owned()))
    );
    assert_eq!(
        parse_split::<String, _>("a much longer input", RedirectedPattern),
        ["é", "🦀"]
    );
}

#[test]
fn mutable_slice_references_remain_disjoint_in_either_order() {
    for (first, second) in [(0, 2), (2, 0)] {
        let mut values = [String::from("a"), String::from("b"), String::from("c")];

        let (left, right) = values.get_mut_twice(first, second);
        left.push('x');
        right.push('y');
        std::mem::swap(left, right);
        left.push('z');

        assert_eq!(values[first].as_bytes().last(), Some(&b'z'));
        assert_eq!(values[second].as_bytes().last(), Some(&b'x'));
    }
}

#[test]
fn mutable_zero_sized_elements_are_supported() {
    let mut values = [(); 2];

    let (left, right) = slice_get_mut_twice(&mut values, 1, 0);
    *left = ();
    *right = ();
}

#[test]
#[should_panic]
fn mutable_slice_rejects_out_of_bounds_indices() {
    let mut values = [1, 2];

    slice_get_mut_twice(&mut values, 0, 2);
}

#[test]
#[should_panic(expected = "SStr bytes must be valid UTF-8")]
fn truncated_unicode_cannot_produce_an_invalid_str() {
    let value = SStr::<1>::from("é");

    value.as_str();
}

#[test]
fn fixed_strings_support_unicode_and_zero_capacity() {
    let unicode = SStr::<6>::from("é🦀");
    let empty = SStr::<0>::from("é🦀");

    assert_eq!(unicode.as_str(), "é🦀");
    assert_eq!(format!("{unicode:?}"), "é🦀");
    assert_eq!(empty.as_str(), "");
    assert!(empty.is_empty());
    assert!(!empty.starts_with_byte(0));
}

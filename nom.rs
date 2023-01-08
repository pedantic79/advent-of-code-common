use std::{fmt::Debug, ops::RangeFrom};

use nom::{
    bytes::complete::take,
    combinator::{all_consuming, map, opt},
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    sequence::terminated,
    AsChar, Compare, CompareResult, IResult, InputIter, InputLength, InputTake, Parser, Slice,
};

/// parser for `usize` that is a  wrapper around [u64](https://docs.rs/nom/latest/nom/character/complete/fn.u64.html).
/// This is only defined on 64-bit systems.
#[cfg(target_pointer_width = "64")]
pub fn nom_usize<T>(s: T) -> IResult<T, usize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength,
    <T as InputIter>::Item: AsChar,
{
    map(nom::character::complete::u64, |x| x as usize)(s)
}

/// parser for `isize` that is a  wrapper around [i64](https://docs.rs/nom/latest/nom/character/complete/fn.i64.html).
/// This is only defined on 64-bit systems.
#[cfg(target_pointer_width = "64")]
pub fn nom_isize<T>(s: T) -> IResult<T, isize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
    <T as InputIter>::Item: AsChar,
    T: for<'a> Compare<&'a [u8]>,
{
    map(nom::character::complete::i64, |x| x as isize)(s)
}

macro_rules! uints {
    ($($n:ident, $t:tt)+) => {
        $(
            #[inline(always)]
            pub fn $n<T>(s: T) -> IResult<T, $t>
            where
                T: InputIter + Slice<RangeFrom<usize>> + InputLength,
                <T as InputIter>::Item: AsChar,
            {
                nom::character::complete::$t(s)
            }
        )+
    };
}

uints! { nom_u8,u8 nom_u16,u16 nom_u32,u32 nom_u64,u64 nom_u128,u128}

macro_rules! ints {
    ($($n:ident, $t:tt)+) => {
        $(
            #[inline(always)]
            pub fn $n<T, E: ParseError<T>>(s: T) -> IResult<T, $t, E>
                where
                T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
                <T as InputIter>::Item: AsChar,
                T: for <'a> Compare<&'a[u8]>,
            {
                nom::character::complete::$t(s)
            }
        )+
    };
}

ints! { nom_i8,i8 nom_i16,i16 nom_i32,i32 nom_i64,i64 nom_i128,i128}

pub trait NewLine {
    fn get_newline() -> Self;
}

impl NewLine for &str {
    fn get_newline() -> Self {
        "\n"
    }
}

impl NewLine for &[u8] {
    fn get_newline() -> Self {
        b"\n"
    }
}

fn nl<I, E>(s: I) -> IResult<I, I, E>
where
    I: NewLine + Compare<I> + InputIter + InputTake + Clone,
    E: ParseError<I>,
{
    let (remainder, ch) = take(1_usize)(s.clone())?;

    if ch.compare(I::get_newline()) != CompareResult::Ok {
        Err(nom::Err::Error(E::from_error_kind(s, ErrorKind::Not)))
    } else {
        Ok((remainder, ch))
    }
}

pub fn process_input<F, I, R, E>(mut f: F) -> impl FnMut(I) -> R
where
    I: NewLine + Compare<I> + InputIter + InputTake + Clone + InputLength,
    F: Parser<I, R, E>,
    E: ParseError<I> + Debug,
{
    move |i: I| {
        all_consuming(optional_trailing_nl(|x| f.parse(x)))
            .parse(i)
            .unwrap()
            .1
    }
}

pub fn optional_trailing_nl<F, I, R, E>(mut f: F) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: NewLine + Compare<I> + InputIter + InputTake + Clone,
    F: Parser<I, R, E>,
    E: ParseError<I>,
{
    move |i: I| terminated(|x| f.parse(x), opt(nl)).parse(i)
}

pub fn nom_lines<F, I, R, E>(mut f: F) -> impl FnMut(I) -> IResult<I, Vec<R>, E>
where
    I: NewLine + Compare<I> + InputIter + InputTake + Clone + InputLength,
    F: Parser<I, R, E>,
    E: ParseError<I>,
{
    move |i: I| separated_list0(nl, |x| f.parse(x)).parse(i)
}

pub fn separated_fold0<I, S, SO, F, FO, C, J, R, E>(
    mut sep: S,
    mut f: F,
    mut init: J,
    mut combiner: C,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, FO, E>,
    S: Parser<I, SO, E>,
    E: ParseError<I>,
    C: FnMut(R, FO) -> R,
    J: FnMut() -> R,
{
    move |mut i: I| {
        let mut res = init();

        match f.parse(i.clone()) {
            Err(nom::Err::Error(_)) => return Ok((i, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res = combiner(res, o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res = combiner(res, o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

pub fn separated_fold1<I, S, SO, F, FO, C, J, R, E>(
    mut sep: S,
    mut f: F,
    mut init: J,
    mut combiner: C,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, FO, E>,
    S: Parser<I, SO, E>,
    E: ParseError<I>,
    C: FnMut(R, FO) -> R,
    J: FnMut() -> R,
{
    move |mut i: I| {
        let mut res = init();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res = combiner(res, o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res = combiner(res, o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

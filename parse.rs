use std::{fmt::Debug, str::FromStr};

pub fn parse_pair<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, ',')
}

pub fn parse_range<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, "..")
}

/// Parse the first `N` pieces, returning `None` if too few pieces or a parse fails.
///
/// Custom patterns may substitute their own haystack. Splitting uses that
/// string instead of `s`.
pub fn parse_split_n<'a, const N: usize, T, P>(s: &'a str, p: P) -> Option<[T; N]>
where
    T: FromStr,
    P: stable_pattern::Pattern<'a>,
{
    pattern::split(s, p)
        .take(N)
        .map(|x| x.parse::<T>().ok())
        .collect::<Option<arrayvec::ArrayVec<T, N>>>()
        .and_then(|o| o.into_inner().ok())
}

/// Split at the first match and parse both sides, returning `None` on failure.
///
/// Custom patterns may substitute their own haystack. Splitting uses that
/// string instead of `s`.
pub fn parse_split_once<'a, T, P>(s: &'a str, p: P) -> Option<(T, T)>
where
    T: FromStr,
    P: stable_pattern::Pattern<'a>,
{
    let (l, r) = pattern::split_once(s, p)?;

    let l = l.parse().ok()?;
    let r = r.parse().ok()?;
    Some((l, r))
}

pub fn parse_lines_fn<T>(s: &str, f: impl Fn(&str) -> T) -> Vec<T> {
    s.lines().map(f).collect()
}

pub fn parse_lines<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_lines_fn(s, |line| line.parse().unwrap())
}

/// Split and parse every piece.
///
/// Custom patterns may substitute their own haystack. Splitting uses that
/// string instead of `s`.
///
/// # Panics
///
/// Panics if any piece fails to parse as `T`.
pub fn parse_split<'a, T, P>(s: &'a str, p: P) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    P: stable_pattern::Pattern<'a>,
{
    pattern::split(s, p)
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

mod pattern {
    use stable_pattern::{Pattern, Searcher, Split, SplitInternal};

    pub fn split_once<'a, P: Pattern<'a>>(s: &'a str, delimiter: P) -> Option<(&'a str, &'a str)> {
        let mut matcher = delimiter.into_searcher(s);
        let (start, end) = matcher.next_match()?;
        let haystack = matcher.haystack();

        Some((&haystack[..start], &haystack[end..]))
    }

    pub fn split<'a, P: Pattern<'a>>(s: &'a str, pat: P) -> Split<'a, P> {
        let matcher = pat.into_searcher(s);
        // Searcher guarantees valid indices into its own haystack. Pattern is
        // safe to implement and need not preserve the string passed to it.
        let end = matcher.haystack().len();

        Split(SplitInternal {
            start: 0,
            end,
            matcher,
            allow_trailing_empty: true,
            finished: false,
        })
    }
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("10,20"), Some((10, 20)));
        assert_eq!(parse_pair::<u32>("foo,bar"), None);
        assert_eq!(parse_pair::<i32>("10"), None);
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range::<i32>("10..20"), Some((10, 20)));
        assert_eq!(parse_range::<u32>("10..bar"), None);
    }

    #[test]
    fn test_parse_split_once() {
        assert_eq!(parse_split_once::<i32, _>("10-20", "-"), Some((10, 20)));
        assert_eq!(parse_split_once::<i32, _>("10-", "-"), None);
    }

    #[test]
    fn test_parse_split_n() {
        let res: Option<[i32; 3]> = parse_split_n("1,2,3", ",");
        assert_eq!(res, Some([1, 2, 3]));

        let res_fail: Option<[i32; 3]> = parse_split_n("1,2", ",");
        assert_eq!(res_fail, None);

        let res_invalid: Option<[i32; 3]> = parse_split_n("1,2,foo", ",");
        assert_eq!(res_invalid, None);
    }

    #[test]
    fn test_parse_lines() {
        let input = "10\n20\n30";
        assert_eq!(parse_lines::<i32>(input), vec![10, 20, 30]);

        let custom = parse_lines_fn(input, |x| x.parse::<i32>().unwrap() * 2);
        assert_eq!(custom, vec![20, 40, 60]);
    }

    #[test]
    fn test_parse_split() {
        assert_eq!(parse_split::<i32, _>("1|2|3", "|"), vec![1, 2, 3]);
    }

    #[test]
    fn test_pattern_split_and_split_once() {
        assert_eq!(pattern::split_once("foo-bar", "-"), Some(("foo", "bar")));
        assert_eq!(pattern::split_once("foobar", "-"), None);

        let parts: Vec<&str> = pattern::split("a,b,c", ",").collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
}

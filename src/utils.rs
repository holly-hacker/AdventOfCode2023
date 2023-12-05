#![allow(unused)]

/// Parses an integer string
///
/// This is faster than `str::parse` because it assumes the input consists of only digits.
pub fn fast_parse_int(s: &str) -> usize {
    debug_assert!(!s.is_empty());
    debug_assert!(s.len() < usize::MAX.to_string().len()); // err on the side of caution
    debug_assert!(s.chars().all(|c| c == ' ' || c.is_ascii_digit()));

    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

/// Parses an int string with a max length of 8
///
/// This is faster than `str::parse` because it assumes the input consists of only digits and is up
/// to 8 characters long.
pub fn fast_parse_int_from_bytes(s: &[u8]) -> usize {
    debug_assert!(!s.is_empty());
    debug_assert!(s.len() < usize::MAX.to_string().len()); // err on the side of caution
    debug_assert!(s.iter().all(|b| *b == b' ' || b.is_ascii_digit()));

    s.iter().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

/// Equivalent to `str`'s `split_once` function, but for byte slices.
#[inline]
pub fn split_once(haystack: &[u8], delimiter: u8) -> Option<(&[u8], &[u8])> {
    let start = haystack.iter().position(|b| *b == delimiter)?;
    // SAFETY: `position` is known to return valid indices.
    unsafe {
        Some((
            haystack.get_unchecked(..start),
            haystack.get_unchecked((start + 1)..),
        ))
    }
}

/// A variant of [`split_once`] that stops at either the first occurance of the delimiter, or at
/// the end of the string.
#[inline]
pub fn split_once_2(haystack: &[u8], delimiter: u8) -> (&[u8], &[u8]) {
    // TODO: position instead?
    // haystack.split_at(haystack.iter().take_while(|&&b| b != delimiter).count())
    let start = haystack
        .iter()
        .position(|b| *b == delimiter)
        .unwrap_or(haystack.len());
    // SAFETY: `position` is known to return valid indices. `haystack.len()` would return an empty slice.
    unsafe {
        (
            haystack.get_unchecked(..start),
            haystack.get_unchecked(start..),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_parse_int() {
        assert_eq!(fast_parse_int("1"), 1);
        assert_eq!(fast_parse_int("0"), 0);
        assert_eq!(fast_parse_int("1234"), 1234);
        assert_eq!(fast_parse_int("0001"), 1);
        assert_eq!(fast_parse_int("0000"), 0);
        assert_eq!(fast_parse_int("12345678"), 12345678);
        assert_eq!(fast_parse_int(" 1"), 1); // handle leading whitespace
    }

    #[test]
    fn test_fast_parse_int_from_bytes() {
        assert_eq!(fast_parse_int_from_bytes(b"1"), 1);
        assert_eq!(fast_parse_int_from_bytes(b"0"), 0);
        assert_eq!(fast_parse_int_from_bytes(b"1234"), 1234);
        assert_eq!(fast_parse_int_from_bytes(b"0001"), 1);
        assert_eq!(fast_parse_int_from_bytes(b"0000"), 0);
        assert_eq!(fast_parse_int_from_bytes(b"12345678"), 12345678);
        assert_eq!(fast_parse_int_from_bytes(b" 1"), 1); // handle leading whitespace
    }

    #[test]
    fn test_split_once() {
        assert_eq!(
            split_once(b"1-1", b'-'),
            Some((b"1".as_ref(), b"1".as_ref()))
        );
        assert_eq!(split_once(b"1-", b'-'), Some((b"1".as_ref(), b"".as_ref())));
        assert_eq!(split_once(b"-1", b'-'), Some((b"".as_ref(), b"1".as_ref())));
        assert_eq!(split_once(b"", b'-'), None);
        assert_eq!(split_once(b"aaa", b'-'), None);
        assert_eq!(
            split_once(b"aaa", b'a'),
            Some((b"".as_ref(), b"aa".as_ref()))
        );
    }
}

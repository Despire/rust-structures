//! # Strings
//!
//! `strings` provide utilities functions for strings

/// Splits a string into tokens
///
/// A sequence of calls to this function split s into tokens
/// separated by delim.
///
/// # Examples
///
/// ```
/// use strings::strtok;
///
/// let mut s = "hello world goodbye!";
///
/// let first = strtok(&mut s, ' ');
/// assert_eq!(first, "hello");
///
/// let second = strtok(&mut s, ' ');
/// assert_eq!(second, "world");
///
/// let third = strtok(&mut s, ' ');
/// assert_eq!(third, "goodbye!");
///
/// let other = strtok(&mut s, ' ');
/// assert_eq!(other, "goodbye!");
/// ```
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delim: char) -> &'b str {
    let pos = s.find(delim);
    if let None = pos {
        return s;
    }

    let pos = pos.unwrap();
    let result = &s[..pos];
    *s = &s[pos + delim.len_utf8()..];

    result
}

use std::fmt;
use std::fmt::Write;

/// Escape a string to pass it into JavaScript.
///
/// # Example
///
/// ```rust,no_run
/// # use tether::Tether;
/// # use std::mem;
/// #
/// # let win: Tether = unsafe { mem::uninitialized() };
/// #
/// let string = "Hello, world!";
///
/// // Calls the function callback with "Hello, world!" as its parameter.
///
/// win.eval(&format!("callback({});", tether::escape(string)));
/// ```
pub fn escape(string: &str) -> Escaper {
    Escaper(string)
}

// "All code points may appear literally in a string literal except for the
// closing quote code points, U+005C (REVERSE SOLIDUS), U+000D (CARRIAGE
// RETURN), U+2028 (LINE SEPARATOR), U+2029 (PARAGRAPH SEPARATOR), and U+000A
// (LINE FEED)." - ES6 Specification

#[doc(hidden)]
pub struct Escaper<'a>(&'a str);

const SPECIAL: &'static [char] = &[
    '\n', // U+000A (LINE FEED)
    '\r', // U+000D (CARRIAGE RETURN)
    '\'', // U+0027 (APOSTROPHE)
    '\\', // U+005C (REVERSE SOLIDUS)
    '\u{2028}', // U+2028 (LINE SEPARATOR)
    '\u{2029}', // U+2029 (PARAGRAPH SEPARATOR)
];

impl<'a> fmt::Display for Escaper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Escaper(mut string) = self;

        f.write_char('\'')?;

        while string.len() != 0 {
            if let Some(i) = string.find(SPECIAL) {
                if i > 0 {
                    f.write_str(&string[..i])?;
                }

                f.write_str(match string[i..].chars().next().unwrap() { //TODO: This line is gross.
                    '\n' => "\\n",
                    '\r' => "\\r",
                    '\'' => "\\'",
                    '\\' => "\\\\",
                    '\u{2028}' => "\\u2028",
                    '\u{2029}' => "\\u2029",
                    _ => unreachable!()
                })?;

                string = &string[i + 1..];
            } else {
                f.write_str(string)?;
                break;
            }
        }

        f.write_char('\'')?;

        Ok(())
    }
}

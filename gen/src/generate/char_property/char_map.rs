pub use std::collections::BTreeMap;
use std::fmt;

/// A mapping from character codepoints to properties.
///
/// Implementations are expected to (although not required to) be naive storage that
/// compacts to the indicated format when calling an endpoint provided by this trait.
///
/// The strings created by the format functions are designed to be used as "`.rsv`" (Rust Value)
/// generated source files and imported with the `include!` macro.
pub trait CharMap<T: Eq> {
    /// A simple deduplicated binary search array slice.
    ///
    /// Output format:
    ///
    /// ```text
    /// &[
    ///     ('low', 'high', Value),
    ///     ('low', 'high', Value),
    /// ]
    /// ```
    ///
    /// Where
    ///
    /// - `'low'` is a `char::escape_unicode` literal for the lowest character in the range
    /// - `'high'` is a `char::escape_unicode` literal for the highest character in the range
    /// - `Value` is the result of running `display_fn` over the associated value
    ///
    /// It is guaranteed that the `'high'` of one range will always be less than the `'low'` of
    /// the next range (such that the array slice is fit for a binary search). The ranges
    /// represented by `'low'` and `'high'` are inclusive on both ends.
    fn to_bsearch_table<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: fmt::Display;

    /// A simple default for when the associated value aready impls `fmt::Display`.
    ///
    /// Intended to be used when the associated value is a string representing the desired
    /// Rust expression output.
    fn to_bsearch_table_default(&self) -> String
    where
        for<'a> &'a T: fmt::Display,
    {
        // TODO: Is it possible to remove the format call here?
        self.to_bsearch_table(|t| format!("{}", t))
    }
}

impl<T: Eq> CharMap<T> for BTreeMap<char, T> {
    fn to_bsearch_table<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: fmt::Display,
    {
        let mut entries = self.iter();
        let mut out = String::from("&[\n");

        let first = entries.next();
        if first.is_none() {
            return out + "]";
        }

        let (mut low, mut value) = first.unwrap();
        let mut high = low;

        for (char, property) in entries {
            if property != value || (*char as u32) > (*high as u32 + 1) {
                append_triple(&mut out, *low, *high, &display_fn(value));
                low = char;
                high = char;
                value = property;
            } else {
                assert_eq!(*char as u32, *high as u32 + 1);
                high = char;
            }
        }

        append_triple(&mut out, *low, *high, &display_fn(value));
        out.push_str("]");
        out
    }
}

fn append_triple<T>(str: &mut String, a: char, b: char, c: &T)
where
    T: fmt::Display,
{
    str.push_str(&format!(
        "    ('{}', '{}', {}),\n",
        a.escape_unicode(),
        b.escape_unicode(),
        c,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_map_bsearch_table() {
        let mut map: BTreeMap<char, &'static str> = Default::default();
        map.insert('a', "Low");
        map.insert('b', "Low");
        map.insert('c', "Low");
        map.insert('d', "Mid");
        map.insert('y', "High");
        map.insert('f', "Mid");
        map.insert('e', "Mid");
        map.insert('x', "High");
        map.insert('z', "High");
        assert_eq!(
            map.to_bsearch_table_default(),
            "\
&[
    ('\\u{61}', '\\u{63}', Low),
    ('\\u{64}', '\\u{66}', Mid),
    ('\\u{78}', '\\u{7a}', High),
]"
        );
    }
}

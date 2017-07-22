pub use std::collections::BTreeSet;

/// A set of character codepoints.
///
/// Implementations are expected to (although not required to) be naive storage that
/// compacts to the indicated format when calling an endpoint provided by this trait.
///
/// The strings created by the format functions are designed to be used as "`.rsv`" (Rust Value)
/// generated source files and imported with the `include!` macro.
pub trait CharSet {
    /// A simple deduplicated binary search array slice.
    ///
    /// Output format:
    ///
    /// ```text
    /// &[
    ///     ('low', 'high'),
    ///     ('low', 'high'),
    /// ]
    /// ```
    ///
    /// Where
    ///
    /// - `'low'` is a `char::escape_unicode` literal for the lowest character in the range
    /// - `'high'` is a `char::escape_unicode` literal for the highest character in the range
    ///
    /// It is guaranteed that the `'high'` of one range will always be less than the `'low'` of
    /// the next range (such that the array slice is fit for a binary search). The ranges
    /// represented by `'low'` and `'high'` are inclusive on both ends.
    fn to_bsearch_table(&self) -> String;
}

impl CharSet for BTreeSet<char> {
    fn to_bsearch_table(&self) -> String {
        let mut entries = self.iter();
        let mut out = String::from("&[\n");

        let first = entries.next();
        if first.is_none() {
            return out + "]";
        }

        let mut low = first.unwrap();
        let mut high = low;

        for char in entries {
            if (*char as u32) > (*high as u32 + 1) {
                append_duple(&mut out, *low, *high);
                low = char;
                high = char;
            } else {
                assert_eq!(*char as u32, *high as u32 + 1);
                high = char;
            }
        }

        append_duple(&mut out, *low, *high);
        out.push_str("]");
        out
    }
}

fn append_duple(str: &mut String, a: char, b: char) {
    str.push_str(&format!(
        "    ('{}', '{}'),\n",
        a.escape_unicode(),
        b.escape_unicode(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_set_bsearch_table() {
        let mut set: BTreeSet<char> = Default::default();
        set.insert('a');
        set.insert('b');
        set.insert('c');
        set.insert('d');
        set.insert('y');
        set.insert('f');
        set.insert('e');
        set.insert('x');
        set.insert('z');
        assert_eq!(
            set.to_bsearch_table(),
            "\
&[
    ('\\u{61}', '\\u{66}'),
    ('\\u{78}', '\\u{7a}'),
]"
        );
    }
}

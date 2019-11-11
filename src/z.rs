//! Exact string matching by Z algorithm.
//!
//! Z algorithm takes advantage of *Z-array*.
//! Z-array of string `S` is a sequence of integers with the same length as `S`
//! satisfing the following condition:
//! for all `i` (0 ≤ `i` < length of `S`),
//! `S[i]` is the maximum `l` such that `S[..l]` matches `S[i..i+l]`,
//! with one exception that `S[0]` = 0 (in case 0 < `i`).
//!
//! Exact string matching is performed by the following steps:
//!
//! 1. For the given text string `T` and pattern string `P`,
//!    constructs `S`, which is a concatenation of `P`, `$` and `T`.
//!    `$` is a character contained by neither `T` nor `P`.
//! 2. Gets the Z-array of `S`.
//! 3. Find all elements in the Z-array whose value is the length of `P`.
//!
//! For example, let `T` be `aaabbbabbaabb` and `P` be `aabb`.
//! Then `S` and the Z-array is constructed as follows:
//!
//! <pre>
//!       S: a a b b $ a a a b b b a b b a a b b
//! Z-array: 0 1 0 0 0 2 4 1 0 0 0 1 0 0 4 1 0 0
//! </pre>
//!
//! So the algorithm find 2 occurrences of `P` in `T`.

use std;

// BEGIN SNIPPET z

mod z_internal {
    use std::ops::Range;

    /// Finds maximum `l` that `&s1[..l]` and `&s2[..l]` are the same sequence.
    pub fn exact_match_len<T: Eq>(s1: &[T], s2: &[T]) -> usize {
        s1.iter().zip(s2).take_while(|&(c1, c2)| c1 == c2).count()
    }

    /// Core of Z algorithm.
    /// See: Dan Gusfield, 1997, *Algorithms on Strings, Trees and Sequences*, p. 9.
    pub fn update_state<T: Eq>(
        text: &[T], z_box: &mut Range<usize>, z_table: &mut Vec<usize>
    ) -> bool {
        let index = z_table.len();

        if index >= text.len() {
            return false;
        }

        if z_box.end <= index {
            let len = exact_match_len(text, unsafe { text.get_unchecked(index..) });
            *z_box = index..index+len;
            z_table.push(len);
        } else {
            let z_box_right_len = z_box.end - index;
            let prefix_index = index - z_box.start;
            let prefix_right_len = *unsafe { z_table.get_unchecked(prefix_index) };
            if prefix_right_len < z_box_right_len {
                z_table.push(prefix_right_len);
            } else {
                let additional_len = exact_match_len(
                    unsafe { text.get_unchecked(z_box_right_len..) },
                    unsafe { text.get_unchecked(z_box.end..) }
                );
                *z_box = index .. z_box.end + additional_len;
                z_table.push(z_box_right_len + additional_len);
            }
        }

        true
    }
}

/// An iterator generated by
/// [`longest_prefix_lengths`](trait.ZString.html#tymethod.longest_prefix_lengths).
pub struct LongestPrefixLengths<'a, T: 'a + Eq> {
    text: &'a [T],
    z_box: std::ops::Range<usize>,
    z_table: Vec<usize>,
}

impl<'a, T: Eq> LongestPrefixLengths<'a, T> {
    fn new(text: &'a [T]) -> LongestPrefixLengths<'a, T> {
        let mut z_table = Vec::with_capacity(text.len());
        z_table.push(0);
        LongestPrefixLengths {
            text: text,
            z_box: 0..0,
            z_table: z_table
        }
    }
}

impl<'a, T: Eq> Iterator for LongestPrefixLengths<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        use self::z_internal::*;

        if update_state(self.text, &mut self.z_box, &mut self.z_table) {
            Some(*self.z_table.last().unwrap())
        } else {
            None
        }
    }
}

/// An iterator generated by [`z_match_indices`](trait.ZString.html#tymethod.z_match_indices).
pub struct ZMatchIndices<T: Eq + Clone> {
    concat: Vec<T>,
    pattern_len: usize,
    z_box: std::ops::Range<usize>,
    z_table: Vec<usize>
}

impl<T: Eq + Clone> Iterator for ZMatchIndices<T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        use self::z_internal::*;

        while update_state(&self.concat, &mut self.z_box, &mut self.z_table) {
            if *self.z_table.last().unwrap() == self.pattern_len {
                return Some(self.z_table.len() - 1 - (self.pattern_len + 1))
            }
        }
        None
    }
}

impl<T: Eq + Clone> ZMatchIndices<T> {
    fn new<I: Iterator<Item=T>>(pattern: Vec<T>, sep: T, text: I) -> ZMatchIndices<T> {
        use self::z_internal::*;

        let len = pattern.len();
        let mut concat = pattern;
        concat.push(sep);
        concat.extend(text);
        let mut z_box = 0..0;
        let mut z_table = vec![0];

        for _ in 0..len {
            update_state(&concat, &mut z_box, &mut z_table);
        }

        ZMatchIndices {
            concat: concat,
            pattern_len: len,
            z_box: z_box,
            z_table: z_table,
        }
    }
}

/// A string (sequence) equipped with methods for Z-array and exact string matching.
pub trait ZString<T: Eq + Clone> {
    /// Gets an iterator yielding Z-array's values.
    ///
    /// Definition of Z-array is stated in the [module documentation](index.html).
    /// The iterator yields all Z-array's values except for the 1st one (always 0).
    ///
    /// Exahausting the iterator takes Θ(`self.len()`) time.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::z::*;
    /// let text: Vec<char> = "abababaab".chars().collect();
    /// let mut indices = text.longest_prefix_lengths();
    ///
    /// assert_eq!(indices.next(), Some(0)); // match length for "abababaab" and "bababaab"
    /// assert_eq!(indices.next(), Some(5)); // match length for "abababaab" and "ababaab"
    /// assert_eq!(indices.next(), Some(0)); // match length for "abababaab" and "babaab"
    /// assert_eq!(indices.next(), Some(3)); // match length for "abababaab" and "abaab"
    /// assert_eq!(indices.next(), Some(0)); // match length for "abababaab" and "baab"
    /// assert_eq!(indices.next(), Some(1)); // match length for "abababaab" and "aab"
    /// assert_eq!(indices.next(), Some(2)); // match length for "abababaab" and "ab"
    /// assert_eq!(indices.next(), Some(0)); // match length for "abababaab" and "b"
    /// assert_eq!(indices.next(), None);
    /// ```
    ///
    /// In some problem, you will takes advantage of this preprosessing
    /// rather than exact string matching.
    ///
    /// For example, to solve
    /// [Codeforces Round #578 (Div. 2), Problem E - Compress Words](https://codeforces.com/contest/1200/problem/E),
    /// you have to find the length of the longest string,
    /// which is both `S`'s suffix and `T`'s prefix, in O(length of `T`) time.
    /// This can be implemented using `longest_prefix_length`.
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::z::*;
    /// use std::cmp;
    ///
    /// fn length(s: &str, t: &str) -> usize {
    ///     let s: Vec<char> = s.chars().collect();
    ///     let t: Vec<char> = t.chars().collect();
    ///
    ///     let len = cmp::min(s.len(), t.len());
    ///     let s_trimmed = &s[s.len() - len..];
    ///     let t_trimmed = &t[..len];
    ///
    ///     // Let s_trimmed be "sample" and t_trimmed be "please".
    ///     // Then concat becomes "pleasesample".
    ///     let concat: Vec<char> = t_trimmed.iter().cloned()
    ///         .chain(s_trimmed.iter().cloned())
    ///         .collect();
    ///
    ///     // The last 6 elements of the Z-array is [0, 0, 0, 3, 0, 0].
    ///     // The length to be returned is 3
    ///     // because it's the 3rd element of the Z-array counted from the end of the array,
    ///     // so first 3 characters in t_trimmed match the last 3 characters in s_trimmed.
    ///     concat.longest_prefix_lengths().skip(len-1).enumerate()
    ///         .find(|&(i, l)| len - i == l)
    ///         .map(|(_, l)| l)
    ///         .unwrap_or(0)
    /// }
    ///
    /// assert_eq!(length("sample", "please"), 3);
    /// assert_eq!(length("samplease", "ease"), 4);
    /// assert_eq!(length("samplease", "in"), 0);
    /// assert_eq!(length("sampleasein", "out"), 0);
    /// ```
    fn longest_prefix_lengths(&self) -> LongestPrefixLengths<T>;

    /// Gets an iterator yielding `self`'s indices matching `pattern`.
    ///
    /// Exahausting the iterator takes Θ(`self.len() + pattern.len()`) time.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::z::*;
    /// let text: Vec<char> = "xababxabababxabxabab".chars().collect();
    /// let pattern: Vec<char> = "abab".chars().collect();
    /// let mut indices = text.z_match_indices(&pattern);
    ///
    /// // "xababxabababxabxabab" matches "abab" at position 1, 6, 8 and 16.
    /// assert_eq!(indices.next(), Some(1));
    /// assert_eq!(indices.next(), Some(6));
    /// assert_eq!(indices.next(), Some(8));
    /// assert_eq!(indices.next(), Some(16));
    /// assert_eq!(indices.next(), None);
    /// ```
    fn z_match_indices(&self, pattern: &Self) -> ZMatchIndices<Option<T>>;
}

impl<T: Eq + Clone> ZString<T> for [T] {
    fn longest_prefix_lengths(&self) -> LongestPrefixLengths<T> {
        LongestPrefixLengths::new(self)
    }

    fn z_match_indices(&self, pattern: &[T]) -> ZMatchIndices<Option<T>> {
        let mut copied = Vec::with_capacity(self.len() + 1 + pattern.len());
        copied.extend(pattern.iter().map(|x| Some(x.clone())));
        ZMatchIndices::new(copied, None, self.iter().map(|x| Some(x.clone())))
    }
}

// END SNIPPET

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_prefix_lengths() {
        let lengths: Vec<usize> = Vec::<char>::new().longest_prefix_lengths().collect();
        assert_eq!(lengths, vec![]);

        let lengths: Vec<usize> = ['a'].longest_prefix_lengths().collect();
        assert_eq!(lengths, vec![]);

        let lengths: Vec<usize> = ['a', 'a'].longest_prefix_lengths().collect();
        assert_eq!(lengths, vec![1]);

        let lengths: Vec<usize> = ['a', 'b'].longest_prefix_lengths().collect();
        assert_eq!(lengths, vec![0]);

        let text: Vec<char> = "aabcabaabcac".chars().collect();
        let lengths: Vec<usize> = text.longest_prefix_lengths().collect();
        assert_eq!(lengths, vec![1, 0, 0, 1, 0, 5, 1, 0, 0, 1, 0]);
    }

    #[test]
    fn test_z_match_indices() {
        let text: Vec<char> = vec![];
        let pattern: Vec<char> = vec![];
        let indices: Vec<usize> = text.z_match_indices(&pattern).collect();
        assert_eq!(indices, vec![]);

        let text: Vec<char> = vec![];
        let pattern = vec!['a'];
        let indices: Vec<usize> = text.z_match_indices(&pattern).collect();
        assert_eq!(indices, vec![]);

        let text = vec!['a'];
        let pattern: Vec<char> = vec![];
        let indices: Vec<usize> = text.z_match_indices(&pattern).collect();
        assert_eq!(indices, vec![0]);

        let text = vec!['a'; 10];
        let pattern: Vec<char> = vec![];
        let indices: Vec<usize> = text.z_match_indices(&pattern).collect();
        assert_eq!(indices, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let text = vec!['a'];
        let pattern = vec!['a'];
        let indices: Vec<usize> = text.z_match_indices(&pattern).collect();
        assert_eq!(indices, vec![0]);
    }
}

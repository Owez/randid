//! Randid (pronounced like random but with `-id` instead of `-om`) is a minimalistic,
//! web-safe library for generating customisable IDs for use in primarily web
//! applications. The generated IDs are not guarenteed to be unique however.
//!
//! ## Common functions
//!
//! | Overview                                 | Function signature   | Example call + response      |
//! |------------------------------------------|----------------------|------------------------------|
//! | Random BASE62 string of exact length     | randid_str(len: i32) | `randid_str(5)` -> `"bWk9D"` |
//! | Random padded i32 string of exact length | randid_i32(len: i32) | `randid_int(5)` -> `"00396"` |

use rand::{self, Rng};

/// Array of
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Generates a random BASE62 [String] of a given length.
///
/// For example, if you provide a length of `5` you will get 5 random BASE62 characters
/// contained in the resulting [String].
///
/// This function uses [BASE62](https://www.wikidata.org/wiki/Q809817) (62 unique
/// characters) as opposed to the more commonly used
/// [BASE64](https://en.wikipedia.org/wiki/Base64) due to the high likelyhood of
/// this function being used for URLs.
///
/// ## Examples
///
/// ```rust
/// use randid::randid_str;
///
/// fn main() {
///     let my_id = randid_str(5);
///
///     println!("https://example.com/safeid/{}", my_id); // will provide a url-safe id like `bWk9D`, `yWvm3` or `POf3R`
/// }
/// ```
pub fn randid_str(len: i32) -> String {
    let mut generated = String::with_capacity(len as usize);

    let mut rng = rand::thread_rng();

    for _ in 0..len {
        generated.push(BASE62[rng.gen::<usize>() % 62] as char);
    }

    generated
}

/// Generates a random padded [i32]-based [String] according to the length.
///
/// This function automatically finds the minimum and maximum integer for the given
/// length. For example, if you input a length of `4` you can get anything between
/// `"0000"` and `"9999"`.
///
/// # Examples
///
/// ```rust
/// use randid::randid_i32;
///
/// fn main() {
///     let padded_num_12 = randid_i32(12);
///     let padded_num_24 = randid_i32(24);
///
///     println!(
///         "Guarenteed length of 12: {}, Guarenteed length of 24: {}",
///         padded_num_12,
///         padded_num_24
///     );
/// }
/// ```
pub fn randid_i32(len: i32) -> String {
    let mut generated = String::with_capacity(len as usize);

    for _ in 0..len {
        let num = rand::thread_rng().gen_range(0, 9);

        generated.push_str(&num.to_string()); // NOTE: probably not most efficiant
    }

    generated
}

#[cfg(test)]
mod tests {
    use super::*;

    /// String length test for [randid_str]
    #[test]
    fn rand_str_len() {
        let result: String = randid_str(10);

        assert_eq!(10, result.len());
    }

    /// Checks the number given by the [randid_i32] is within the correct range
    /// asked for
    #[test]
    fn rand_int_range() {
        let (min, max) = (0, 99999999);

        let result: i32 = randid_i32(8).parse().unwrap();

        assert!(min <= result);
        assert!(result <= max);
    }
}

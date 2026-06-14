//! This library uses the authors from Anarchist Library to generate a random name
//!
//! The intention here is to have a random name for situations you need to spin up some cloud compute resources and don't have a name in mind. Typically, for throwaway purposes.
//!
//! # Examples
//!
//! It's possible to simply generate a random name
//!
//! ```
//! use anarchist_readable_name_generator_lib::readable_name;
//!
//! assert!(!readable_name().is_empty())
//! ```
//!
//! You can also pass a seed or change the separator to give you predictability or minor customization.
//!
//! ```
//! use anarchist_readable_name_generator_lib::readable_name_custom;
//! use rand::prelude::*;
//! use rand_chacha::ChaChaRng;
//!
//! let rng = ChaChaRng::seed_from_u64(2);
//! assert_eq!(
//!    readable_name_custom("+", rng),
//!    "proficient+kamalmaz"
//! );
//! ```

#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]

mod names;

use names::ADJECTIVES;
use names::NAMES;
use rand::Rng;
use rand::seq::IndexedRandom;

/// Generate a readable name with some customization
///
/// # Examples
///
/// ```
/// use anarchist_readable_name_generator_lib::readable_name_custom;
/// use rand::prelude::*;
/// use rand_chacha::ChaChaRng;
///
/// let rng = ChaChaRng::seed_from_u64(2);
/// assert_eq!(
///    readable_name_custom("+", rng),
///    "proficient+kamalmaz"
/// );
/// ```
///
/// # Panics
///
/// Should not panic, would panic if there were no ADJECTIVES or if NAMES (both constants guaranteed not to be empty)
#[must_use]
pub fn readable_name_custom<R: Rng>(separator: &str, mut rng: R) -> String {
    format!(
        "{}{}{}",
        ADJECTIVES
            .choose(&mut rng)
            .expect("This should never fail, our list is predefined"),
        separator,
        NAMES
            .choose(&mut rng)
            .expect("This should never fail, our list is predefined")
    )
}

/// Generate a readable name with a custom separator and a random numeric suffix
///
/// # Examples
///
/// ```
/// use anarchist_readable_name_generator_lib::{readable_name_custom_suffix};
/// use rand::prelude::*;
/// use rand_chacha::ChaChaRng;
///
/// let rng = ChaChaRng::seed_from_u64(2);
/// assert_eq!(
///    readable_name_custom_suffix("+", rng),
///    "dynamic+lepper3"
/// );
/// ```
///
/// # Panics
///
/// Should not panic, would panic if there were no ADJECTIVES or if NAMES (both constants guaranteed not to be empty)
#[must_use]
pub fn readable_name_custom_suffix<R: Rng>(separator: &str, mut rng: R) -> String {
    let suffix = rng.random_range(0..=9);

    format!("{}{}", readable_name_custom(separator, &mut rng), suffix)
}

/// Generate a readable name using the default underscore separator
///
/// # Examples
///
/// ```
/// use anarchist_readable_name_generator_lib::readable_name;
///
/// assert!(!readable_name().is_empty())
/// ```
#[must_use]
pub fn readable_name() -> String {
    let rng = rand::rng();
    readable_name_custom("_", rng)
}

#[cfg(test)]
mod test_readable_name_custom {
    use super::{readable_name, readable_name_custom, readable_name_custom_suffix};

    use rand::prelude::*;
    use rand_chacha::ChaChaRng;

    #[test]
    fn it_generates_a_name_with_a_custom_separator() {
        let rng = rand::rng();
        let split = readable_name_custom("-", rng)
            .split('-')
            .map(String::from)
            .collect::<Vec<_>>();
        assert!(!split.first().unwrap().is_empty());
        assert!(!split.get(1).unwrap().is_empty());
        assert_eq!(split.len(), 2);
    }

    #[test]
    fn it_can_be_made_predictable_with_a_known_seed() {
        let rng_1 = ChaChaRng::seed_from_u64(2);
        let rng_2 = ChaChaRng::seed_from_u64(2);
        assert_eq!(
            readable_name_custom("_", rng_1),
            readable_name_custom("_", rng_2)
        );
    }

    #[test]
    fn it_can_add_a_random_number_to_the_end_to_make_it_unique() {
        let rng_1 = ChaChaRng::seed_from_u64(2);
        assert_eq!(readable_name_custom_suffix("_", rng_1), "dynamic_lepper3");
    }

    #[test]
    fn readable_name_uses_default_underscore_separator() {
        let name = readable_name();
        let parts: Vec<&str> = name.split('_').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }

    #[test]
    fn readable_name_custom_suffix_ends_with_digit() {
        let rng = rand::rng();
        let name = readable_name_custom_suffix("_", rng);
        let last_char = name.chars().last().expect("name should not be empty");
        assert!(last_char.is_ascii_digit());
    }

    #[test]
    fn adjectives_has_no_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for adj in crate::names::ADJECTIVES {
            assert!(seen.insert(*adj), "duplicate adjective: {adj}");
        }
    }

    #[test]
    fn names_has_no_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for name in crate::names::NAMES {
            assert!(seen.insert(*name), "duplicate name: {name}");
        }
    }
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            unsafe extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

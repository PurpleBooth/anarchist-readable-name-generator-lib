mod names;

use names::ADJECTIVES;
use names::NAMES;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[must_use]
pub fn readable_name_custom<R: Rng>(seperator: &str, mut rng: R) -> String {
    format!(
        "{}{}{}",
        ADJECTIVES
            .choose(&mut rng)
            .expect("This should never fail, our list is predefined"),
        seperator,
        NAMES
            .choose(&mut rng)
            .expect("This should never fail, our list is predefined")
    )
}

#[must_use]
pub fn readable_name() -> String {
    let rng = thread_rng();
    readable_name_custom("_", rng)
}

#[cfg(test)]
mod test_readable_name_custom {
    use super::readable_name_custom;

    use rand::prelude::*;
    use rand_pcg::Pcg64;

    #[test]
    fn it_generates_a_name_with_a_custom_separator() {
        let rng = thread_rng();
        let split = readable_name_custom("-", rng)
            .split('-')
            .map(String::from)
            .collect::<Vec<_>>();
        assert!(!split.get(0).unwrap().is_empty());
        assert!(!split.get(1).unwrap().is_empty());
        assert_eq!(split.len(), 2);
    }

    #[test]
    fn it_can_be_made_predictable_with_a_known_seed() {
        let rng_1 = Pcg64::seed_from_u64(2);
        let rng_2 = Pcg64::seed_from_u64(2);
        assert_eq!(
            readable_name_custom("_", rng_1),
            readable_name_custom("_", rng_2)
        );
    }
}

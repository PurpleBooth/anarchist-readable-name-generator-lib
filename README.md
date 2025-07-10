# anarchist-readable-name-generator-lib

This library uses the authors from [Anarchist
Library](https://theanarchistlibrary.org/special/index) to generate a
random name

The intention here is to have a random name for situations you need to
spin up some cloud compute resources and don't have a name in mind.
Typically, for throwaway purposes.

## Examples

It's possible to simply generate a random name

```rust
    use anarchist_readable_name_generator_lib::readable_name;

assert!(readable_name().len() > 0);
```

You can also pass a seed or change the separator to give you
predictability or minor customization.

```rust
    use anarchist_readable_name_generator_lib::readable_name_custom;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

let rng = ChaChaRng::seed_from_u64(2);
assert_eq!(
    readable_name_custom("+", rng),
    "romantic+kamalmaz"
);
```

You can also increase entropy a little by suffixing a random number

```rust
 use anarchist_readable_name_generator_lib::readable_name_custom_suffix;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

let rng = ChaChaRng::seed_from_u64(2);
assert_eq!(
    readable_name_custom_suffix("+", rng),
    "dynamic+lepper3"
);
```

Read more at
[Docs.rs](https://docs.rs/anarchist-readable-name-generator-lib/)

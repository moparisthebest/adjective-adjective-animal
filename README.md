# adjective_adjective_animal

[![Build Status](https://ci.moparisthe.best/job/moparisthebest/job/adjective-adjective-animal/job/master/badge/icon%3Fstyle=plastic)](https://ci.moparisthe.best/job/moparisthebest/job/adjective-adjective-animal/job/master/) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/moparisthebest/adjective-adjective-animal/blob/master/LICENSE-MIT)

Rust library to generate suitably random and reasonably unique human readable (and fairly adorable) ids,
ala GiphyCat

* Crate: https://crates.io/crates/adjective_adjective_animal
* Documentation https://docs.rs/crate/adjective_adjective_animal
* Source Code: https://github.com/moparisthebest/adjective-adjective-animal

## Usage

This crate is [on crates.io](https://crates.io/crates/adjective_adjective_animal) and can be
used by adding `adjective_adjective_animal` to your dependencies in your project's `Cargo.toml`
file:

```toml
[dependencies]
adjective_adjective_animal = "0.1.0"
```

and this to your crate root:

```rust
extern crate adjective_adjective_animal;
```

### Example: Painless defaults

The easiest way to get started is to use the default `Generator` to return
a name:

```rust
use adjective_adjective_animal::Generator;

fn main() {
    let mut generator = Generator::default();
    println!("Your project is: {}", generator.next().unwrap());
    // #=> "Your project is: IndustrialSecretiveSwan"
    
}

```

# Example: with custom dictionaries

If you would rather supply your own custom adjective and animal word lists,
you can provide your own by supplying 2 string slices. For example,
this returns only one result:

```
use adjective_adjective_animal::Generator;

fn main() {
    let adjectives = &["Imaginary"];
    let animals = &["Bear"];
    let mut generator = Generator::new(adjectives, animals);

    assert_eq!("ImaginaryImaginaryBear", generator.next().unwrap());
}
```

# Credits
  * rust's [names](https://github.com/fnichol/names) crate, which this is forked from
  * npm's [adjective-adjective-animal](https://github.com/a-type/adjective-adjective-animal) for lists
     * `curl 'https://raw.githubusercontent.com/a-type/adjective-adjective-animal/master/lib/lists/animals.js' | grep -Eo '"[^"]+"' | tr -d '"' | tr '[:upper:]' '[:lower:]' | sed 's/.*/\u&/' | sort | uniq > animals.txt`
     * `curl 'https://raw.githubusercontent.com/a-type/adjective-adjective-animal/master/lib/lists/adjectives.js' | grep -Eo '"[^"]+"' | tr -d '"' | tr '[:upper:]' '[:lower:]' | sed 's/.*/\u&/' | sort | uniq > adjectives.txt`

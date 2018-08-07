//! This crate provides a generate that constructs suitably random and reasonably unique human
//! readable (and fairly adorable) ids, ala GiphyCat.
//!
//! The name `Generator` implements the `Iterator` trait so it can be used with
//! adapters, consumers, and in loops.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/adjective_adjective_animal) and can be
//! used by adding `adjective_adjective_animal` to your dependencies in your project's `Cargo.toml`
//! file:
//!
//! ```toml
//! [dependencies]
//! adjective_adjective_animal = "0.9.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate adjective_adjective_animal;
//! ```
//!
//! # Example: painless defaults
//!
//! The easiest way to get started is to use the default `Generator` to return
//! a name:
//!
//! ```
//! use adjective_adjective_animal::Generator;
//!
//! let mut generator = Generator::default();
//! println!("Your project is: {}", generator.next().unwrap());
//! // #=> "Your project is: IndustrialSecretiveSwan"
//! ```
//!
//! # Example: with custom dictionaries
//!
//! If you would rather supply your own custom adjective and animal word lists,
//! you can provide your own by supplying 2 string slices. For example,
//! this returns only one result:
//!
//! ```
//! use adjective_adjective_animal::Generator;
//!
//! let adjectives = &["Imaginary"];
//! let animals = &["Bear"];
//! let mut generator = Generator::new(adjectives, animals);
//!
//! assert_eq!("ImaginaryImaginaryBear", generator.next().unwrap());
//! ```

extern crate rand;

use rand::Rng;

pub const ADJECTIVES: &'static [&'static str] =
    &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

pub const ANIMALS: &'static [&'static str] = &include!(concat!(env!("OUT_DIR"), "/animals.rs"));

/// A random name generator which combines an adjective, a animal, and an
/// optional number
///
/// A `Generator` takes a slice of adjective and animal words strings and has
/// a naming strategy (with or without a number appended).
pub struct Generator<'a> {
    adjectives: &'a [&'a str],
    animals: &'a [&'a str],
}

impl<'a> Generator<'a> {
    /// Constructs a new `Generator<'a>`
    ///
    /// # Examples
    ///
    /// ```
    /// use adjective_adjective_animal::Generator;
    ///
    /// let adjectives = &["Sassy"];
    /// let animals = &["Fox"];
    ///
    /// let mut generator = Generator::new(adjectives, animals);
    ///
    /// assert_eq!("SassySassyFox", generator.next().unwrap());
    /// ```
    pub fn new(adjectives: &'a [&'a str], animals: &'a [&'a str]) -> Self {
        Generator {
            adjectives: adjectives,
            animals: animals,
        }
    }

    fn rand_adj(&self) -> &str {
        rand::thread_rng().choose(self.adjectives).unwrap()
    }

    fn rand_animal(&self) -> &str {
        rand::thread_rng().choose(self.animals).unwrap()
    }
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Generator::new(ADJECTIVES, ANIMALS)
    }
}

impl<'a> Iterator for Generator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(format!("{}{}{}", self.rand_adj(), self.rand_adj(), self.rand_animal()))
    }
}

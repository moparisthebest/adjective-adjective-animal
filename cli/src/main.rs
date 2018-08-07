#[macro_use]
extern crate clap;
extern crate adjective_adjective_animal;

use adjective_adjective_animal::Generator;

fn main() {
    let amount = {
        let app =
            clap_app!(aaa =>
                (version: &crate_version!()[..])
                (author: "\nAuthor: Travis Burtrum <admin.github@moparisthebest.com>\n")
                (about: "A random name generator with results like `DeliriousHungryDolphin'.")
                (@setting ColoredHelp)
                (@arg amount: "Number of names to generate (default: 1)")
        );
        let matches = app.get_matches();
        value_t!(matches.value_of("amount"), usize).unwrap_or(1)
    };

    let mut generator = Generator::default();
    for _ in 0..amount {
        println!("{}", generator.next().unwrap());
    }
}

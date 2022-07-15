use clap::{App, Arg};

fn main() {
    let _matches = App::new("am")
        .version("0.1.0")
        .author("Todd W Crone <twcrone@gmail.com>")
        .about("Alias Manager")
        .arg(
            Arg::with_name("alias")
                .value_name("ALIAS")
                .help("Alias name")
                .max_values(1),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .help("Delete the alias")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", _matches)
}

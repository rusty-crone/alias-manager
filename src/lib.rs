use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let matches = App::new("am")
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
                .short('d')
                .help("Delete the alias")
                .takes_value(false),
        )
        .get_matches();

    let alias_option = matches.value_of_lossy("alias");
    if alias_option.is_some() {
        let alias = alias_option.unwrap();
        let delete = matches.is_present("delete");
        if delete {
            print!("delete ")
        }
        else {
            print!("add/update ")
        }
        println!("{}", alias)
    }
    else {
        println!("list aliases")
    }
    Ok(())
}
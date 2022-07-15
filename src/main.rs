use clap::App;

fn main() {
    let _matches = App::new("ram")
        .version("0.1.0")
        .author("Todd W Crone <twcrone@gmail.com>")
        .about("Alias Manager")
        .get_matches();
}


fn main() {
    if let Err(e) = am::run() {
        eprint!("{}", e);
        std::process::exit(1)
    }
}

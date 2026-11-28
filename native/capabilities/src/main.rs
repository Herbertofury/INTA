fn main() {
    let (exit, response) = inta_capabilities::invoke(std::env::args_os().skip(1));
    println!("{response}");
    std::process::exit(exit);
}

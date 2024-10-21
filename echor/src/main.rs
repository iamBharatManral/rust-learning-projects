fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    echor::run(&args);
}

fn main() {
    if let Err(e) = jq::run() {
        panic!(e);
    }
}

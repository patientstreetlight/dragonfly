// From https://github.com/jeremyletang/rust-sfml/wiki/MacOS
// Thanks Kes 🙂
fn main() {
    println!("cargo::rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo::rustc-link-arg=-ObjC");
}

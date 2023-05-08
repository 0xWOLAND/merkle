mod bitops;
mod sha256;

fn main() {
    sha256::hash(&mut String::from("a"));
    sha256::hash(&mut String::from("b"));
}

use rust_bin::{message, version};

fn main() {
    println!("{}", message());
    println!("continuous-release-bazel {}", version());
}

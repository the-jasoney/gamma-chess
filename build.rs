use std::process::Command;

fn main() {
    println!("running cargo-fmt");
    Command::new("cargo").arg("fmt");
}

use std::process::Command;
fn main() {
    let out = Command::new("sh")
        .arg("-c")
        .arg("echo $ABC $DEF")
        .envs([("ABC", "aaa"), ("DEF", "ddd")])
        .status();
    println!("{:?}", out)
}

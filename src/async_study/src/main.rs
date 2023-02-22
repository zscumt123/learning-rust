use anyhow::{anyhow, Result};
use serde_yaml::Value;
use std::{
    fs,
    io::Read,
    thread::{self, JoinHandle},
};

struct MyJoinHandle<T>(JoinHandle<Result<T>>);
impl<T> MyJoinHandle<T> {
    pub fn thread_wait(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("failed"))?
    }
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)?;
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        Ok::<_, anyhow::Error>(content)
    });
    MyJoinHandle(handle)
}

fn main() -> Result<()> {
    let content1 = fs::read_to_string("../src/async_study/Cargo.toml")?;
    // let content2 = fs::read_to_string("./cargo.lock")?;

    let yaml1 = toml2yaml(&content1)?;
    // let yaml2 = toml2yaml(&content2)?;
    fs::write("/tmp/cargo.yml", &yaml1)?;
    // // fs::write("/tmp/cargo.lock", &yaml2)?;
    println!("{}", yaml1);
    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}

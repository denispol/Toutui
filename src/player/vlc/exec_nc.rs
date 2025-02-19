use std::process::Command;
use std::process::Output;

pub async fn exec_nc(port: &str) -> Output {
    let output: Output = Command::new("kitty")
        .arg("nc")
        .arg("localhost")
        .arg(format!("{}", port))
        .output()
        .expect("Failed to execute program");

    output
}


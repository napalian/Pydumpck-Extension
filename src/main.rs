use std::env;
use std::process::Command;
use std::io::{self, prelude::*};

fn main() {
    println!("Enter Full .exe Path: ");

    let mut file_path = String::new();

    io::stdin().read_line(&mut file_path)
        .expect("Error Entering Input!");

    let output = Command::new("pydumpck")
        .arg(file_path.trim())
        .args(&["-p", "uncompyle6", "pycdc"])
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
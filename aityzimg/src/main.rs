use std::{io::Write, path::PathBuf};

fn main() {
    println!("Cloning the Git repository!");

    std::process::Command::new("git")
        .args([
            "clone",
            "https://github.com/Aityzimg/Aityzimg.github.io.git",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    print!("What is the name of the file: ");

    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let input_path = PathBuf::from(buffer.trim());

    if !input_path.exists() {
        println!("File does not exist!");
        return;
    }

    print!("Which directory (/, /pics etc): ");

    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let path = PathBuf::from("aityzimg.github.io")
        .join(buffer)
        .join(input_path.file_name().unwrap());

    if path.exists() {
        std::fs::copy(input_path, path).unwrap();
    }

    std::process::Command::new("git")
        .args(["add", "."])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::process::Command::new("git")
        .args(["commit", "-m", "automatic"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::process::Command::new("git")
        .args([
            "git",
            "push",
            "https://github.com/Aityzimg/Aityzimg.github.io",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

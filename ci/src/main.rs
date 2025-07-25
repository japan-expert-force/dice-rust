use std::process::Command;

fn main() {
    let tasks = vec![
        ("fmt", vec!["cargo", "fmt"]),
        ("check", vec!["cargo", "check"]),
        ("test", vec!["cargo", "test"]),
        ("clippy", vec!["cargo", "clippy"]),
        ("build", vec!["cargo", "build", "--release"]),
        (
            "install cargo-audit",
            vec!["cargo", "install", "cargo-audit", "--root", "target/tools"],
        ),
        ("audit", vec!["./target/tools/bin/cargo-audit", "audit"]),
    ];

    for (name, command) in tasks {
        println!("[{}] > {}", name, command.join(" "));
        let status = Command::new(&command[0])
            .args(&command[1..])
            .status()
            .expect("Failed to execute command");
        if !status.success() {
            panic!(
                "Command `{}` failed with status: {}",
                command.join(" "),
                status
            );
        }
    }
}

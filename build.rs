use std::process::Command;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    if profile == "debug" {
        Command::new("typeshare")
            .arg(".")
            .arg("--lang=typescript")
            .arg("--output-file=client/src/lib/types.ts")
            .output()
            .expect("Failed to execute command");
    }
}

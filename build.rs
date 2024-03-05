use std::process::Command;

fn main() {
    Command::new("typeshare")
        .arg(".")
        .arg("--lang=typescript")
        .arg("--output-file=client/src/lib/types.ts")
        .output()
        .expect("Failed to execute command");
}

use std::process::Command;

fn main() {
    // Compile the .rc file to .res
    let output = Command::new("windres")
        .args(&["app.rc", "-O", "coff", "-o", "app.res"])
        .output()
        .expect("Failed to compile resource file");

    if !output.status.success() {
        panic!("windres failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Tell Cargo to link the .res file
    println!("cargo:rustc-link-arg=app.res");
}
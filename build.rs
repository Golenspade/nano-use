fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Ok(output) = std::process::Command::new("xcode-select").arg("-p").output() {
        let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !xcode_path.is_empty() {
            println!(
                "cargo:rustc-link-arg=-Wl,-rpath,{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx"
            );
        }
    }
}

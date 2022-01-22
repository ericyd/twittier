use std::process;

// The only reason we have a custom build script is to expose the git SHA as a variable to the build.
// Worth it? no...
fn main() {
    if let Some(rev) = git_revision_hash() {
        println!("cargo:rustc-env=BUILD_GIT_HASH={}", rev);
    }
}

// Thanks 🙏 https://github.com/BurntSushi/ripgrep/blob/0b36942f680bfa9ae88a564f2636aa8286470073/build.rs#L53-L65
fn git_revision_hash() -> Option<String> {
    let result = process::Command::new("git")
        .args(&["rev-parse", "--short=10", "HEAD"])
        .output();
    result.ok().and_then(|output| {
        let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}

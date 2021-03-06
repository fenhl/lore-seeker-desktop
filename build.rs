use std::{
    fs::File,
    io::{
        self,
        prelude::*
    },
    process::Command
};
#[cfg(windows)]
use winres::WindowsResource;

/// Modified from <https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program>
fn get_git_hash() -> String {
    let commit = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("HEAD")
        .output();
    match commit {
        Ok(commit_output) => {
            let commit_string = String::from_utf8_lossy(&commit_output.stdout);
            commit_string.lines().next().expect("Incorrect formatting of git commit").to_owned()
        }
        Err(e) => { panic!("Can not get git commit: {}", e); }
    }
}

fn main() -> Result<(), io::Error> {
    println!("cargo:rerun-if-changed=nonexistent.foo"); // check a nonexistent file to make sure build script is always run (see https://github.com/rust-lang/cargo/issues/4213)
    {
        let mut f = File::create("src/version.rs")?;
        writeln!(f, "//! Contains versioning information.")?;
        writeln!(f, "")?;
        writeln!(f, "/// The hash of the current commit of the lore-seeker-desktop repo at compile time.")?;
        writeln!(f, "pub const GIT_COMMIT_HASH: &str = \"{}\";", get_git_hash())?;
    } // close src/version.rs
    #[cfg(windows)] {
        let mut res = WindowsResource::new();
        res.set_icon("assets/lore-seeker.ico");
        res.compile()?;
    }
    Ok(())
}

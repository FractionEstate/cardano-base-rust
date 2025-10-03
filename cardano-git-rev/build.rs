use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const ZERO_REV: &str = "0000000000000000000000000000000000000000";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let embedded = git_rev_from_env()
        .and_then(promote_real_rev)
        .or_else(|| run_git_rev_parse().ok().and_then(promote_real_rev))
        .unwrap_or_else(|| ZERO_REV.to_string());

    let dest = out_dir.join("git-rev.txt");
    fs::write(&dest, &embedded).expect("failed to write git revision");

    println!("cargo:rustc-env=CARDANO_GIT_REV={}", embedded);
}

fn git_rev_from_env() -> Option<String> {
    env::var("CARDANO_GIT_REV")
        .ok()
        .map(|value| sanitize(&value))
}

fn promote_real_rev(rev: String) -> Option<String> {
    if is_real_rev(&rev) {
        Some(rev)
    } else {
        None
    }
}

fn run_git_rev_parse() -> Result<String, GitRevError> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .output()
        .map_err(GitRevError::Spawn)?;

    if output.status.success() {
        let rev = String::from_utf8(output.stdout).map_err(GitRevError::Utf8)?;
        Ok(sanitize(&rev))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(GitRevError::Command(stderr.into()))
    }
}

#[derive(Debug)]
enum GitRevError {
    Spawn(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Command(String),
}

fn sanitize(input: &str) -> String {
    let trimmed = input.trim();
    if is_real_rev(trimmed) {
        trimmed.to_lowercase()
    } else {
        ZERO_REV.to_string()
    }
}

fn is_real_rev(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit()) && value != ZERO_REV
}

impl fmt::Display for GitRevError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitRevError::Spawn(err) => write!(f, "failed to spawn git: {err}"),
            GitRevError::Utf8(err) => write!(f, "git output was not valid UTF-8: {err}"),
            GitRevError::Command(stderr) => write!(f, "git rev-parse failed: {stderr}"),
        }
    }
}

impl std::error::Error for GitRevError {}

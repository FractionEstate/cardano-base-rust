use std::borrow::Cow;
use std::env;
use std::process::Command;

const ZERO_REV: &str = "0000000000000000000000000000000000000000";

/// Expose the embedded git revision.
///
/// The result prefers the value captured at build time. If the embedded value
/// is unavailable (for example when building in environments without git
/// metadata), the function attempts to query `git` at runtime. Should that
/// fail as well, the zero revision is returned.
#[must_use]
pub fn git_rev() -> Cow<'static, str> {
    static EMBEDDED: &str = env!("CARDANO_GIT_REV");

    if is_real_rev(EMBEDDED) {
        return Cow::Borrowed(EMBEDDED);
    }

    match git_rev_runtime() {
        Ok(rev) if is_real_rev(&rev) => Cow::Owned(rev),
        _ => Cow::Borrowed(ZERO_REV),
    }
}

fn git_rev_runtime() -> Result<String, GitRevError> {
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

fn is_real_rev(input: &str) -> bool {
    input != ZERO_REV && input.len() == 40 && input.chars().all(|c| c.is_ascii_hexdigit())
}

fn sanitize(input: &str) -> String {
    let trimmed = input.trim();
    if is_real_rev(trimmed) {
        trimmed.to_lowercase()
    } else {
        ZERO_REV.to_string()
    }
}

/// Errors produced when attempting to discover the git revision at runtime.
#[derive(Debug, thiserror::Error)]
pub enum GitRevError {
    #[error("failed to spawn git: {0}")]
    Spawn(std::io::Error),
    #[error("git output was not valid UTF-8: {0}")]
    Utf8(std::string::FromUtf8Error),
    #[error("git rev-parse failed: {0}")]
    Command(String),
}

#[cfg(not(test))]
#[no_mangle]
pub static mut _cardano_git_rev: [u8; 68] = [
    b'f', b'e', b'g', b'i', b't', b'r', b'e', b'v', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'4', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_real_rev() {
        assert!(is_real_rev("0123456789abcdef0123456789abcdef01234567"));
        assert!(!is_real_rev(ZERO_REV));
        assert!(!is_real_rev("foo"));
    }

    #[test]
    fn sanitize_invalid_input() {
        assert_eq!(sanitize("not-a-sha"), ZERO_REV);
        assert_eq!(sanitize(&format!("{}\n", ZERO_REV)), ZERO_REV);
    }
}

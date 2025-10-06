use std::{borrow::Cow, process::Command, sync::OnceLock};

const ZERO_REV: &str = "0000000000000000000000000000000000000000";

/// Expose the git revision associated with this build.
///
/// The result prefers the value embedded at build time. If that value is
/// unavailable the function attempts to query `git` at runtime. All failures
/// are reported via a warning and fall back to the all-zero revision.
#[must_use]
pub fn git_rev() -> Cow<'static, str> {
    if let Some(embedded) = git_rev_embedded() {
        return Cow::Owned(embedded);
    }

    match git_rev_runtime() {
        Ok(rev) => Cow::Owned(rev),
        Err(err) => {
            emit_warning_once(&err);
            Cow::Borrowed(ZERO_REV)
        },
    }
}

fn git_rev_embedded() -> Option<String> {
    if let Some(symbol) = read_embedded_symbol() {
        if is_real_rev(&symbol) {
            return Some(symbol);
        }

        emit_warning_once(&GitRevError::Invalid(symbol));
    }

    let candidate = env!("CARDANO_GIT_REV");
    if is_real_rev(candidate) {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn read_embedded_symbol() -> Option<String> {
    #[cfg(not(test))]
    unsafe {
        let bytes: [u8; 68] = core::ptr::addr_of!(_cardano_git_rev).read();

        if &bytes[0..2] != b"fe" || &bytes[2..8] != b"gitrev" {
            return None;
        }

        let payload = &bytes[28..];
        let terminator = payload
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(payload.len());
        let candidate = std::str::from_utf8(&payload[..terminator])
            .ok()?
            .trim()
            .to_string();

        Some(candidate)
    }

    #[cfg(test)]
    {
        None
    }
}

fn git_rev_runtime() -> Result<String, GitRevError> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .output()
        .map_err(GitRevError::Spawn)?;

    if output.status.success() {
        let raw = String::from_utf8(output.stdout).map_err(GitRevError::Utf8)?;
        let sanitized = sanitize(&raw);
        if is_real_rev(&sanitized) {
            Ok(sanitized)
        } else {
            Err(GitRevError::Invalid(raw.trim().to_string()))
        }
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
        trimmed.to_string()
    } else {
        ZERO_REV.to_string()
    }
}

fn emit_warning_once(error: &GitRevError) {
    static WARN_ONCE: OnceLock<()> = OnceLock::new();
    WARN_ONCE.get_or_init(|| {
        eprintln!("WARNING: {error}");
    });
}

/// Errors produced when attempting to discover the git revision.
#[derive(Debug, thiserror::Error)]
pub enum GitRevError {
    #[error("failed to spawn git: {0}")]
    Spawn(std::io::Error),
    #[error("git output was not valid UTF-8: {0}")]
    Utf8(std::string::FromUtf8Error),
    #[error("git rev-parse failed: {0}")]
    Command(String),
    #[error("git returned invalid revision: {0}")]
    Invalid(String),
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
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
    use std::ffi::OsString;
    use std::sync::{Mutex, OnceLock};

    struct EnvGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    impl EnvGuard {
        fn new(key: &'static str) -> Self {
            Self {
                key,
                original: std::env::var_os(key),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => unsafe {
                    std::env::set_var(self.key, value);
                },
                None => unsafe {
                    std::env::remove_var(self.key);
                },
            }
        }
    }

    static PATH_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

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

    #[test]
    fn runtime_errors_when_git_unavailable() {
        let _lock = PATH_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("path mutex poisoned");

        let _guard = EnvGuard::new("PATH");
        unsafe {
            std::env::set_var("PATH", "");
        }

        let result = git_rev_runtime();
        assert!(matches!(result, Err(GitRevError::Spawn(_))));
    }
}

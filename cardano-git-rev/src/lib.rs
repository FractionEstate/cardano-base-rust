use std::{
    borrow::Cow,
    process::{Command, Output},
    sync::{Mutex, OnceLock},
};

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
    let stored = {
        let lock = embedded_revision_store()
            .lock()
            .expect("embedded revision mutex poisoned");
        lock.clone()
    };

    if is_real_rev(&stored) {
        return Some(stored);
    }

    let fallback = env!("CARDANO_GIT_REV");
    if stored != fallback {
        emit_warning_once(&GitRevError::Invalid(stored));
    }

    if is_real_rev(fallback) {
        Some(fallback.to_string())
    } else {
        None
    }
}

fn git_rev_runtime() -> Result<String, GitRevError> {
    let output = run_git_command(["rev-parse", "--verify", "HEAD"]).map_err(GitRevError::Spawn)?;

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

fn embedded_revision_store() -> &'static Mutex<String> {
    static EMBEDDED_REVISION: OnceLock<Mutex<String>> = OnceLock::new();
    EMBEDDED_REVISION.get_or_init(|| Mutex::new(env!("CARDANO_GIT_REV").to_string()))
}

/// Override the embedded revision for the lifetime of the returned guard.
///
/// This helper is intended for tests that need to simulate patching the
/// embedded revision at runtime without relying on raw pointer manipulation.
#[must_use]
pub fn set_embedded_revision_for_testing(new_revision: impl Into<String>) -> EmbeddedRevisionGuard {
    let mut slot = embedded_revision_store()
        .lock()
        .expect("embedded revision mutex poisoned");
    let original = slot.clone();
    *slot = new_revision.into();
    EmbeddedRevisionGuard { original }
}

/// Guard that restores the previously embedded revision when dropped.
#[derive(Debug)]
pub struct EmbeddedRevisionGuard {
    original: String,
}

impl Drop for EmbeddedRevisionGuard {
    fn drop(&mut self) {
        let mut slot = embedded_revision_store()
            .lock()
            .expect("embedded revision mutex poisoned");
        *slot = self.original.clone();
    }
}

type GitCommandHook = dyn Fn(&[&str]) -> Result<Output, std::io::Error> + Send + Sync + 'static;

fn git_command_override_store() -> &'static Mutex<Option<Box<GitCommandHook>>> {
    static GIT_COMMAND_OVERRIDE: OnceLock<Mutex<Option<Box<GitCommandHook>>>> = OnceLock::new();
    GIT_COMMAND_OVERRIDE.get_or_init(|| Mutex::new(None))
}

fn run_git_command(args: impl IntoIterator<Item = &'static str>) -> Result<Output, std::io::Error> {
    let guard = git_command_override_store()
        .lock()
        .expect("git command override mutex poisoned");
    if let Some(hook) = guard.as_ref() {
        let collected: Vec<&str> = args.into_iter().collect();
        return hook(&collected);
    }
    drop(guard);
    Command::new("git").args(args).output()
}

/// Temporarily override the command used to query git.
///
/// Intended strictly for tests to simulate error scenarios. The override is
/// removed once the returned guard is dropped.
#[must_use]
pub fn override_git_command_for_testing<F>(hook: F) -> GitCommandOverrideGuard
where
    F: Fn(&[&str]) -> Result<Output, std::io::Error> + Send + Sync + 'static,
{
    let mut slot = git_command_override_store()
        .lock()
        .expect("git command override mutex poisoned");
    assert!(slot.is_none(), "git command override already set");
    *slot = Some(Box::new(move |args: &[&str]| hook(args)));
    GitCommandOverrideGuard { restored: false }
}

/// Guard restoring the default git command behaviour when dropped.
#[derive(Debug)]
pub struct GitCommandOverrideGuard {
    restored: bool,
}

impl Drop for GitCommandOverrideGuard {
    fn drop(&mut self) {
        if !self.restored {
            let mut slot = git_command_override_store()
                .lock()
                .expect("git command override mutex poisoned");
            *slot = None;
            self.restored = true;
        }
    }
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

    #[test]
    fn runtime_errors_when_git_unavailable() {
        let _guard = override_git_command_for_testing(|_| {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "git unavailable",
            ))
        });

        let result = git_rev_runtime();
        assert!(matches!(result, Err(GitRevError::Spawn(_))));
    }

    #[test]
    fn allows_overriding_embedded_revision() {
        let baseline = git_rev().into_owned();
        let replacement = "0123456789abcdef0123456789abcdef01234567";
        let alternate = "89abcdef0123456789abcdef0123456701234567";
        let desired = if baseline == replacement {
            alternate
        } else {
            replacement
        };

        {
            let _guard = set_embedded_revision_for_testing(desired.to_string());
            let rev = git_rev();
            assert_eq!(rev.as_ref(), desired);
        }

        let restored = git_rev().into_owned();
        assert_eq!(restored, baseline);
    }
}

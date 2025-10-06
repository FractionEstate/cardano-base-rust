use cardano_git_rev::git_rev;

unsafe extern "C" {
    static mut _cardano_git_rev: [u8; 68];
}

fn is_hex_sha(candidate: &str) -> bool {
    candidate.len() == 40 && candidate.chars().all(|c| c.is_ascii_hexdigit())
}

struct StaticGuard([u8; 68]);

impl Drop for StaticGuard {
    fn drop(&mut self) {
        unsafe {
            core::ptr::addr_of_mut!(_cardano_git_rev).write(self.0);
        }
    }
}

fn replace_embedded(revision: &str) -> StaticGuard {
    assert_eq!(revision.len(), 40, "revision must be a 40-character SHA1");

    let original = unsafe { core::ptr::addr_of!(_cardano_git_rev).read() };
    let mut patched = original;
    patched[28..68].copy_from_slice(revision.as_bytes());

    unsafe {
        core::ptr::addr_of_mut!(_cardano_git_rev).write(patched);
    }

    StaticGuard(original)
}

#[test]
fn git_rev_returns_sane_value() {
    let rev = git_rev();
    assert!(is_hex_sha(&rev), "unexpected git revision: {rev}");
}

#[test]
fn patch_placeholder_layout() {
    let bytes: [u8; 68] = unsafe { core::ptr::addr_of!(_cardano_git_rev).read() };
    assert_eq!(&bytes[0..2], b"fe");
    assert_eq!(&bytes[2..8], b"gitrev");
    assert_eq!(&bytes[8..18], b"0000000000");
    assert_eq!(&bytes[18..28], b"0000000040");
}

#[test]
fn prefers_injected_embedded_revision() {
    let replacement = "0123456789abcdef0123456789abcdef01234567";
    let _guard = replace_embedded(replacement);

    let rev = git_rev();
    assert_eq!(rev.as_ref(), replacement);
}

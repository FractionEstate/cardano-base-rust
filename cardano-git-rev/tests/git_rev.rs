use cardano_git_rev::{git_rev, set_embedded_revision_for_testing};

fn is_hex_sha(candidate: &str) -> bool {
    candidate.len() == 40 && candidate.chars().all(|c| c.is_ascii_hexdigit())
}

#[test]
fn git_rev_returns_sane_value() {
    let rev = git_rev();
    assert!(is_hex_sha(&rev), "unexpected git revision: {rev}");
}

#[test]
fn prefers_overridden_embedded_revision() {
    let baseline = git_rev().into_owned();
    let replacement = "0123456789abcdef0123456789abcdef01234567";
    let alternate = "89abcdef0123456789abcdef0123456701234567";
    let desired = if baseline == replacement {
        alternate
    } else {
        replacement
    };

    {
        let _guard = set_embedded_revision_for_testing(desired);
        let rev = git_rev();
        assert_eq!(rev.as_ref(), desired);
    }

    let restored = git_rev().into_owned();
    assert_eq!(restored, baseline);
}

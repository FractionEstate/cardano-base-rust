use cardano_git_rev::git_rev;

fn is_hex_sha(candidate: &str) -> bool {
    candidate.len() == 40 && candidate.chars().all(|c| c.is_ascii_hexdigit())
}

#[test]
fn git_rev_returns_sane_value() {
    let rev = git_rev();
    assert!(is_hex_sha(&rev), "unexpected git revision: {rev}");
}

#[test]
fn patch_placeholder_layout() {
    unsafe {
        extern "C" {
            static mut _cardano_git_rev: [u8; 68];
        }

        let bytes: [u8; 68] = core::ptr::addr_of!(_cardano_git_rev).read();
        assert_eq!(&bytes[0..2], b"fe");
        assert_eq!(&bytes[2..8], b"gitrev");
        assert_eq!(&bytes[8..18], b"0000000000");
        assert_eq!(&bytes[18..28], b"0000000040");
    }
}

#[cfg(test)]
mod snapshot_tests {
    use std::process::Command;

    use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

    #[test]
    fn test_snapshot() {
        assert_cmd_snapshot!(Command::new(get_cargo_bin("gilded_rose_rust")));
    }
}

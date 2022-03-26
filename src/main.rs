use std::env;

fn main() {
    println!("not yet implemented")
}

mod tests {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_noargs_noflags() {
        let mut subject = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = subject
            .current_dir(env::current_dir().unwrap().join("src/testdata"))
            .write_stdin("noargs")
            .assert();
        let want = include_str!("testdata/noargs/want.txt");
        assert.success().stdout(want);
    }
}

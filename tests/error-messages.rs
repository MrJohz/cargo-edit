use std::process;

fn assert_output_contains(output: &process::Output, needle: &str) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.to_lowercase().find(needle).is_none() && stdout.to_lowercase().find(needle).is_none()  {
        panic!("Match was unsuccessful.\nSTDOUT: {:?}\nSTDERR: {:?}\nNEEDLE: {:?}",
            stdout, stderr, needle);
    }
}


mod cargo_add {
    use super::assert_output_contains;
    use std::process;

    #[test]
    fn invalid_arguments() {
        // - should state that the argument is invalid
        // - should provide usage information
        let call = process::Command::new("target/debug/cargo-add")
            .output().unwrap();

        assert!(!call.status.success());
        assert_output_contains(&call, "invalid argument");
        assert_output_contains(&call, "usage:");


        let call = process::Command::new("target/debug/cargo-add")
            .arg("invalid").arg("arguments").arg("here")
            .output().unwrap();

        assert!(!call.status.success());
        assert_output_contains(&call, "invalid argument");
        assert_output_contains(&call, "usage:");
    }

    #[test]
    fn nonexistant_files() {
        // - should state that file could not be found

        let call = process::Command::new("target/debug/cargo-add")
            .arg("add") /* always necessary */ .arg("pkg")
            .arg("--manifest-path").arg("this-file-doesnt-exist.txt")
            .output().unwrap();

        assert!(!call.status.success());
        assert_output_contains(&call, "no such file or directory");
    }
}

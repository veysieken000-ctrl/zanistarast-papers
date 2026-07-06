use std::process::Command;

#[test]
fn cli_runs_end_to_end_certified_execution() {
    let output = Command::new(env!("CARGO_BIN_EXE_zanistarast-cli"))
        .output()
        .expect("failed to execute zanistarast-cli");

    assert!(
        output.status.success(),
        "CLI process failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8(output.stdout)
        .expect("CLI stdout should be valid UTF-8");

    assert!(stdout.contains("ZANISTARAST END-TO-END CERTIFIED EXECUTION"));
    assert!(stdout.contains("Verification Passed: true"));
    assert!(stdout.contains("Certification Verified: true"));
    assert!(stdout.contains("Publication Status: Published"));
}



use absorb::commands::*;
use std::process::Command;
use git2::{Repository};

fn git_revparse() -> String {
  let output = Command::new("git")
                       .arg("rev-parse")
                       .arg("--abbrev-ref")
                       .arg("HEAD")
                       .output()
                       .unwrap_or_else(|e| {
                          panic!("failed to execute process: {}", e) });

  String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_branch() {
    let path = ".";
    let repo = Repository::open(&path).unwrap();
    let branchname = branch(&repo).unwrap();
    let contrast = git_revparse();

    assert_eq!(contrast.trim_right(), branchname);
}

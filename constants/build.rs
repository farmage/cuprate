fn main() {
    set_commit_env();
}

/// This sets the git `COMMIT` environment variable.
fn set_commit_env() {
    // Cargo already reruns this build script when a source file in this package changes.
    // `../.git/refs/heads/` is not a valid path in a submodule checkout: its `.git` entry is
    // a file pointing at the parent repository's module directory. Watching that nonexistent
    // path marked this crate (and the entire noded dependency chain) dirty on every build.
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");

    let commit = if let Ok(t) = std::env::var("GITHUB_SHA") {
        t
    } else {
        // FIXME: This could also be `std::fs::read({PATH}/{branch})`
        // so the machine building doesn't need `git`, although:
        // 1. Having `git` as a build dependency is probably ok
        // 2. It causes issues on PRs that aren't the `main` branch
        String::from_utf8(
            std::process::Command::new("git")
                .args(["show", "-s", "--format=%H"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
    }
    .trim()
    .to_lowercase();

    assert_eq!(
        commit.len(),
        40,
        "Commit hash should always be 40 bytes long."
    );

    println!("cargo:rustc-env=COMMIT={commit}");
}

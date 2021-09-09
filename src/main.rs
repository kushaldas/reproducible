use shadow_rs::shadow;
shadow!(build);

fn main() {
    println!("Debug build:{}", shadow_rs::is_debug());
    println!("git_clean: {}", shadow_rs::git_clean());

    println!("Branch: {}", build::BRANCH);
    println!("Short commit: {}", build::SHORT_COMMIT);
    println!("Commit HASH: {}", build::COMMIT_HASH);
    println!("Commit date: {}", build::COMMIT_DATE);
    println!("Commit author: {}", build::COMMIT_AUTHOR);
    println!("Commit email: {}", build::COMMIT_EMAIL);

    println!("OS: {}", build::BUILD_OS);
    println!("Rust Version: {}", build::RUST_VERSION);
    println!("Rust channel: {}", build::RUST_CHANNEL);
    println!("Cargo version: {}", build::CARGO_VERSION);

    println!("Package name: {}", build::PROJECT_NAME);
    println!("Package verison: {}", build::PKG_VERSION);
    println!("Build time: {}", build::BUILD_TIME);
}

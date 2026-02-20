use std::process::Command;

fn main() {
    if std::env::var("SKIP_TRUNK").is_ok() {
        return;
    }
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set");
    let status = Command::new("trunk")
        .arg("build")
        .current_dir(&manifest_dir)
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            panic!("trunk build failed with exit code: {:?}", s.code());
        }
        Err(e) => {
            panic!(
                "failed to run trunk build (is trunk installed?): {}",
                e
            );
        }
    }
}

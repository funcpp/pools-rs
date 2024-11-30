use std::time::{SystemTime, UNIX_EPOCH};

fn set_built_at() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("cargo:rustc-env=BUILT_AT={}", since_the_epoch.as_secs());
}

fn main() {
    set_built_at();
}

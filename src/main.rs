use std::process::Command;

use chrono::{Duration, Utc};
use tempfile::TempDir;
// use tough::{Limits, Repository, Settings};

use std::path::{PathBuf};

fn sample_data() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.join("sample").join("data")
}

fn main() {

    println!("--- BEGIN ---");
    let args: Vec<String> = std::env::args().collect();
    let temp_repo_dir = TempDir::new().unwrap();

    let repo_dir =
        if args.len() > 1 {
            args[1].as_str()
        }
        else {
            temp_repo_dir.path().to_str().unwrap()
        };

    // let sample_path = sample_data();
    let timestamp_expiration = Utc::now().checked_add_signed(Duration::days(3)).unwrap();
    let timestamp_version: u64 = 1234;
    let snapshot_expiration = Utc::now().checked_add_signed(Duration::days(21)).unwrap();
    let snapshot_version: u64 = 5432;
    let targets_expiration = Utc::now().checked_add_signed(Duration::days(13)).unwrap();
    let targets_version: u64 = 789;
    let targets_input_dir = sample_data().join("tuf-reference-impl").join("targets");
    let root_json = sample_data().join("simple-rsa").join("root.json");
    let root_key = sample_data().join("snakeoil.pem");

    println!("creating repository at {}...\r\n", repo_dir);

    // Create a repo using tuftool and the reference tuf implementation targets
    Command::new("tuftool")
        .args(&[
            "create",
            targets_input_dir.to_str().unwrap(),
            repo_dir,
            "-k",
            root_key.to_str().unwrap(),
            "--root",
            root_json.to_str().unwrap(),
            "--targets-expires",
            targets_expiration.to_rfc3339().as_str(),
            "--targets-version",
            format!("{}", targets_version).as_str(),
            "--snapshot-expires",
            snapshot_expiration.to_rfc3339().as_str(),
            "--snapshot-version",
            format!("{}", snapshot_version).as_str(),
            "--timestamp-expires",
            timestamp_expiration.to_rfc3339().as_str(),
            "--timestamp-version",
            format!("{}", timestamp_version).as_str(),
        ]).status().expect("failed to create");

    Command::new("ls")
        .arg("-lR")
        .arg(format!("{}",repo_dir))
        .status().expect("couldn't list dir");


}

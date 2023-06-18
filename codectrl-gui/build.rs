use chrono::Utc;
use std::{
    env, fs,
    io::{Read, Write},
    path,
    process::Command,
};
use toml::value::Map;

fn file_not_found(out_dir: &str) {
    let mut versions_file =
        fs::File::create(&path::Path::new(&out_dir).join("versions.include")).unwrap();

    let mut array_str = String::from("pub const build_deps: [(&str, &str); 0] = []");

    if env::var("BUILT_WITH").is_ok() {
        array_str = String::from(
            "pub const build_deps: [(&str, &str); 1] = [(\"Built with Nix\", \"\")]",
        );
    }

    versions_file.write_all(array_str.as_bytes()).unwrap();
}

fn file_found(lock_contents: &str) {
    let lock_toml: Map<String, toml::Value> = toml::from_str(lock_contents).unwrap();

    // Get the table of [[package]]s. This is the deep list of dependencies and
    // dependencies of dependencies.
    let mut packages = Vec::new();

    for package in lock_toml
        .get("package")
        .unwrap()
        .as_array()
        .unwrap()
        .as_slice()
    {
        let package = package.as_table().unwrap();
        packages.push((
            package.get("name").unwrap().as_str().unwrap(),
            package.get("version").unwrap().as_str().unwrap(),
        ));
    }
    packages.sort_unstable();

    // Write out the file to be included in the module stub
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut versions_file =
        fs::File::create(&path::Path::new(&out_dir).join("versions.include")).unwrap();
    versions_file
        .write_all(
            format!(
                "pub const BUILD_DEPS: [(&str, &str); {}] = [",
                packages.len()
            )
            .as_ref(),
        )
        .unwrap();
    for package in packages {
        versions_file
            .write_all(format!("(\"{}\", \"{}\"),\n", package.0, package.1).as_ref())
            .unwrap();
    }
    versions_file.write_all("];".as_ref()).unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Read Cargo.lock and de-toml it
    let mut lock_buf = String::new();

    if let Ok(mut file) = fs::File::open("../Cargo.lock") {
        file.read_to_string(&mut lock_buf).unwrap();
        file_found(&lock_buf);
    } else {
        file_not_found(&out_dir);
    };

    let mut time_file =
        fs::File::create(&path::Path::new(&out_dir).join("build_time.include")).unwrap();
    time_file
        .write_all(
            format!(
                "pub const BUILD_TIME: &str = \"{} ({})\";",
                Utc::now().format("%F %X"),
                Utc::now().timezone()
            )
            .as_ref(),
        )
        .unwrap();

    let output = Command::new("git")
        .args(&["rev-parse", "--short=7", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    let git_branch = String::from_utf8(output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_COMMIT={}", git_hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
}

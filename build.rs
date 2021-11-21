use chrono::Utc;
use std::{
    env, fs,
    io::{Read, Write},
    path,
};
use toml::value::Map;

fn main() {
    // Read Cargo.lock and de-toml it
    let mut lock_buf = String::new();
    fs::File::open("Cargo.lock")
        .unwrap()
        .read_to_string(&mut lock_buf)
        .unwrap();

    let lock_toml: Map<String, toml::Value> = toml::from_str(&lock_buf).unwrap();

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
                "pub const BUILD_DEPS: [(&'static str, &'static str); {}] = [",
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
}

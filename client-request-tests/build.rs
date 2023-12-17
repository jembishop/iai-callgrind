use std::path::PathBuf;

use fs_extra::dir::CopyOptions;

fn set_env_var<K, V>(key: K, value: V)
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    println!("cargo:rustc-env={}={}", key.as_ref(), value.as_ref());
}

fn main() {
    let fixtures = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("Environment variable CARGO_MANIFEST_DIR should exist"),
    )
    .join("tests/fixtures");

    if let Ok(env_var) = std::env::var("CROSS_RUNNER") {
        // cross-runner can be `qemu-user`, `qemu-system`, `native`
        if env_var == "qemu-system" {
            let target = PathBuf::from("/target");
            let fixtures_dest = PathBuf::from(&target).join("fixtures");
            if fixtures_dest.exists() {
                std::fs::remove_dir_all(&fixtures_dest).unwrap();
            }
            fs_extra::copy_items(&[fixtures], &target, &CopyOptions::new()).unwrap();

            set_env_var(
                "CLIENT_REQUEST_TESTS_FIXTURES",
                fixtures_dest.display().to_string(),
            );
        } else {
            set_env_var(
                "CLIENT_REQUEST_TESTS_FIXTURES",
                fixtures.display().to_string(),
            );
        }
    } else {
        set_env_var(
            "CLIENT_REQUEST_TESTS_FIXTURES",
            fixtures.display().to_string(),
        );
    }

    // If Ok, then we're building with cross
    if let Ok(cross_target) = std::env::var("IAI_CALLGRIND_CROSS_TARGET") {
        set_env_var("IAI_CALLGRIND_CROSS_TARGET", cross_target);

        let temp_dir = PathBuf::from(
            std::env::var("IAI_CALLGRIND_CROSS_VALGRIND_TEMPDIR")
                .expect("Environment variable 'IAI_CALLGRIND_CROSS_VALGRIND_TEMPDIR' should exist"),
        );
        let dest_dir = PathBuf::from(
            std::env::var("IAI_CALLGRIND_CROSS_VALGRIND_DESTDIR")
                .expect("Environment variable 'IAI_CALLGRIND_CROSS_VALGRIND_DESTDIR' should exist"),
        );

        if temp_dir.exists() {
            if dest_dir.exists() {
                std::fs::remove_dir_all(dest_dir).unwrap();
            }

            let options = CopyOptions::new(); //Initialize default values for CopyOptions
            fs_extra::copy_items(&[temp_dir], "/", &options).unwrap();
        } else {
            panic!(
                "Temporary valgrind installation path '{}' does not exist",
                temp_dir.display()
            );
        }
    } else {
        set_env_var(
            "IAI_CALLGRIND_CROSS_TARGET",
            std::env::var("TARGET").expect("Environment variable TARGET should be present"),
        );
    }
}

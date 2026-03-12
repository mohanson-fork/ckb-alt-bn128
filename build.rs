use std::io;

fn show_clang_version(clang: &str) -> io::Result<std::process::Output> {
    std::process::Command::new(clang).arg("--version").output()
}

fn find_clang() -> String {
    if let Ok(clang) = std::env::var("CLANG") {
        let clang_version = show_clang_version(&clang);
        if clang_version.is_ok() {
            let s = String::from_utf8_lossy(&clang_version.unwrap().stdout).to_string();
            println!("cargo:warning={}", s);
            return clang;
        }
    }
    for version in [22, 21, 20, 19, 30, 29, 28, 27, 26, 25, 24, 23] {
        let clang = format!("clang-{}", version);
        let clang_version = show_clang_version(&clang);
        if clang_version.is_ok() {
            let s = String::from_utf8_lossy(&clang_version.unwrap().stdout).to_string();
            println!("cargo:warning={}", s);
            return clang;
        }
    }
    let clang_version = show_clang_version("clang");
    if clang_version.is_ok() {
        let s = String::from_utf8_lossy(&clang_version.unwrap().stdout).to_string();
        println!("cargo:warning={}", s);
        return "clang".to_string();
    }
    panic!(
        "Clang compiler not found. Please set the CLANG environment variable to the path of your clang executable."
    );
}

fn main() {
    cc::Build::new()
        .compiler(find_clang())
        .flag("--target=riscv64-unknown-elf")
        .flag("-march=rv64imac")
        .flag("-mabi=lp64")
        .file("src/ll_u256_mont-riscv64.S")
        .compile("ll_u256_mont");
}

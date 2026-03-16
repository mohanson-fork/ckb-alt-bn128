use std::io;
use std::process::Output;

fn show_clang_version(clang: &str) -> io::Result<Output> {
    std::process::Command::new(clang).arg("--version").output()
}

fn find_clang() -> String {
    // The 1st, check if the CLANG environment variable is set and valid.
    if let Ok(clang) = std::env::var("CLANG") {
        match show_clang_version(&clang) {
            Ok(ok) => println!("{}", String::from_utf8_lossy(&ok.stdout)),
            Err(_) => panic!(
                "Clang compiler not found. Error CLANG environment: {}",
                clang
            ),
        }
        return clang;
    }

    // The 2nd, try clang-22, clang-21, ..., clang-19 in order.
    for version in [22, 21, 20, 19, 30, 29, 28, 27, 26, 25, 24, 23] {
        let clang = format!("clang-{}", version);
        match show_clang_version(&clang) {
            Ok(ok) => {
                println!("{}", String::from_utf8_lossy(&ok.stdout));
                return clang;
            }
            Err(_) => continue,
        }
    }

    // The 3rd, try clang.
    match show_clang_version("clang") {
        Ok(ok) => {
            println!("{}", String::from_utf8_lossy(&ok.stdout));
            return "clang".to_string();
        }
        Err(_) => panic!(
            "Clang compiler not found. Please set the CLANG environment variable to the path of your clang executable."
        ),
    }
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

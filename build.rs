fn find_clang() -> String {
    if let Ok(clang) = std::env::var("CLANG") {
        return clang;
    }
    for version in [22, 21, 20, 19, 30, 29, 28, 27, 26, 25, 24, 23] {
        let clang = format!("clang-{}", version);
        if std::process::Command::new(&clang)
            .arg("--version")
            .output()
            .is_ok()
        {
            return clang;
        }
    }
    if std::process::Command::new("clang")
        .arg("--version")
        .output()
        .is_ok()
    {
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

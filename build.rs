fn find_clang() -> String {
    if let Ok(clang) = std::env::var("CLANG") {
        return clang;
    }
    for version in (19..=21).rev() {
        let clang = format!("clang-{}", version);
        if std::process::Command::new(&clang)
            .arg("--version")
            .output()
            .is_ok()
        {
            return clang;
        }
    }
    "clang".to_string()
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

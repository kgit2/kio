fn main() {
    use std::env;
    println!("PROFILE {:?}", env::var("PROFILE"));
    println!("CARGO_CFG_TARGET_OS {:?}", env::var("CARGO_CFG_TARGET_OS"));
    println!(
        "CARGO_CFG_TARGET_ARCH {:?}",
        env::var("CARGO_CFG_TARGET_ARCH")
    );
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_include_guard("FFK_H")
        .rename_item("bool", "int")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/../headers/ffk.h", crate_dir));
}

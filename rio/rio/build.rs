fn main() {
    use std::env;
    println!("PROFILE {:?}", env::var("PROFILE"));
    println!(
        "CARGO_CFG_TARGET_ARCH {:?}",
        env::var("CARGO_CFG_TARGET_ARCH")
    );
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_include_guard("RIO_H")
        // .with_sys_include("stdint.h")
        // .with_sys_include("stdbool.h")
        // .with_no_includes()
        .rename_item("bool", "int")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/../rio.h", crate_dir));
}

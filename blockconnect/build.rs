fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:warning=Output directory: {}", out_dir);

    println!("cargo:rerun-if-changed=schema");

    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .output_path(&out_dir)
        .file("schema/chain.capnp")
        .file("schema/common.capnp")
        .file("schema/echo.capnp")
        .file("schema/handler.capnp")
        .file("schema/init.capnp")
        .file("schema/mining.capnp")
        .file("schema/proxy.capnp")
        .run()
        .expect("compiling schema");
}

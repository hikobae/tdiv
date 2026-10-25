fn main() {
    compile_manifest();
}

fn compile_manifest() {
    let manifest_path = "assets/app.manifest.xml";
    let description =
        std::env::var("CARGO_PKG_DESCRIPTION").expect("CARGO_PKG_DESCRIPTION is not set");

    winresource::WindowsResource::new()
        .set_manifest_file(manifest_path)
        .set("FileDescription", &description)
        .set_icon("assets/app.ico")
        .compile()
        .expect("Failed to compile manifest");
}

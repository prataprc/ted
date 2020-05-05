use cc;

fn main() {
    let source_directory = format!("ts/txt_en/src");
    let source_file = format!("{}/parser.c", source_directory);

    println!("cargo:rerun-if-changed={}", source_file); // <1>

    cc::Build::new()
        .file(source_file)
        .include(source_directory)
        .compile("txt_en"); // <2>
}

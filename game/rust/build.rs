extern crate bindgen;

fn main() {
    println!("AAAAAA\n\n\n\n\nn\n\n\n\n\n");
    let bindings = bindgen::Builder::default()
        .header("../../headers.h")
        .clang_arg("-I../../glad/include/")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    bindings
        .write_to_file("src/bindings.rs")
        .unwrap()
}

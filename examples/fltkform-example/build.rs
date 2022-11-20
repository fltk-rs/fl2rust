fn main() {
    use std::path::PathBuf;
    use std::env;
    println!("cargo:rerun-if-changed=src/mockui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/mockui.fl", out_path.join("mockui.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}
fn main() {
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/openblas/lib");
    // for macos
    println!("cargo:rustc-link-lib=framework=Accelerate");

}
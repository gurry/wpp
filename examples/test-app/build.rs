fn main() {
    #[cfg(feature = "ifr")]
    println!("cargo::rustc-link-lib=dylib=WppRecorderUM");
}
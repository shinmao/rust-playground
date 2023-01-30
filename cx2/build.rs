fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cplusobj.cc")
        .compile("cx2");
}
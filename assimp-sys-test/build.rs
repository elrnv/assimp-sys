extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/tests.cpp")
        .include("../assimp/include")
        .compile("libtests.a");
}

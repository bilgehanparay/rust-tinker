fn main(){
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cpp")
        .compile("cxx-demo");
}
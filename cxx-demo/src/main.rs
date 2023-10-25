
#[cxx::bridge]
mod ffi{
    unsafe extern "C++"{
        include!("cxx-demo/include/blobstore.h");
        type BlobstoreClient;
        /*
        Opaque types may only be manipulated behind an indirection such as a reference &, 
        a Rust Box, or a UniquePtr (Rust binding of std::unique_ptr). 
        We'll add a function through which C++ can return a std::unique_ptr<BlobstoreClient> to Rust.
        */
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
}

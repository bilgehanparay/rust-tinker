/*
An expressive system of opaque types, shared types, 
and key standard library type bindings enables API design 
on the language boundary that captures the proper ownership 
and borrowing contracts of the interface.
*/
#[cxx::bridge]
mod ffi {
    /*
    Anything defiend here is shared
    C++ or Rust can implement(not both)
    */
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        /*
        Anything defined here has to be implemented in RUST
        but accessible from C++
        */
        type MultiBuf;
        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        /*
        Anything defined here has to be implemented in C++
        but accessible from RUST
        */
        include!("cxx-demo/include/blobstore.h");
        /*
        c++ class objeleri rust'a type olarak aktarılıyor
        deklare edilen class methodları da ilk arguman olarak
        self alıyor. mod içinde bir tane type var ise methodun bu
        objeye ait olduğu varsayılıyor. Birden fazla var ise method
        tanımında self: &TypeObject gibi belirtmek gerekiyor.
        self sadece referans olarak geçirilebiliyor
        */
        type BlobstoreClient;
        /* 
        Opaque types may only be manipulated behind an indirection such as a reference &, 
        a Rust Box, or a UniquePtr (Rust binding o
            f std::unique_ptr). 
        We'll add a function through which C++ can return a std::unique_ptr<BlobstoreClient> to Rust.
        */
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        /*
        Any signature having a self parameter (the Rust name for C++'s this) 
        is considered a method / non-static member function
        */
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

// An iterator over contiguous chunks of a discontiguous file object. Toy
// implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
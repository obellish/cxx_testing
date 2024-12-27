use cxx::UniquePtr;

#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {

	struct BlobMetadata {
		size: usize,
		tags: Vec<String>,
	}

	extern "Rust" {
		type MultiBuf;

		fn next_chunk(buf: &mut MultiBuf) -> &[u8];
	}

	unsafe extern "C++" {
		include!("include/blobstore.h");

		type BlobstoreClient;

		fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
	}
}

pub struct MultiBuf {}

fn main() {
	println!("Hello, world!");
}

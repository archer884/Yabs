extern crate filetime;
extern crate rustc_serialize;

use std::path::Path;


mod metadata;
mod workingdirectory;

fn main() {
    println!("Hello, world! ! !");

   let json_path = Path::new("metadata.json");
	metadata::create_emty_metadata_file(json_path);

	let data_path = Path::new("data");
	let hierarchy = workingdirectory::read_working_directory(data_path);

	println!("{:?}", hierarchy);
}

mod model {

	use std::collections::HashMap;

	#[derive(Debug, RustcEncodable, RustcDecodable)]
	pub struct Hierarchy {
		nb_revision: i32,
	    files: HashMap<String, MetaDataSet>
	}

	#[derive(Debug, RustcEncodable, RustcDecodable)]
	pub struct MetaDataSet {
	    metadata: Vec<MetaData>
	}

	#[derive(Debug, RustcEncodable, RustcDecodable)]
	pub struct MetaData {
	   timestamp: u64,
	   size: u64,
	   hash: String,
	   stored_hash: String
	}

	impl Hierarchy {
		pub fn new_empty() -> Hierarchy {
			let empty_hierarchy_map : HashMap<String, MetaDataSet> = HashMap::new();
			Hierarchy {nb_revision: 1, files: empty_hierarchy_map}
		}	
	}

	impl MetaData {
		pub fn new_without_hash(timestamp: u64, size: u64) -> MetaData {
			MetaData {timestamp: timestamp, size: size, hash: "".to_string(), stored_hash: "".to_string()}
		}	
	}

}
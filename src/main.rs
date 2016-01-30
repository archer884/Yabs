extern crate filetime;
extern crate rustc_serialize;

use std::path::Path;
use std::collections::HashMap;


mod metadata;
mod workingdirectory;

fn main() {
    println!("Hello, world! ! !");

   let json_path = Path::new("metadata.json");
	metadata::create_emty_metadata_file(json_path);

	let data_path = Path::new("data");
	let wd_hierarchy = workingdirectory::read_working_directory(data_path);

	let mt_hierarchy = metadata::read_metadata_file(json_path);

	println!("{:?}", mt_hierarchy);

	println!("Finding files to commit");

	let files_to_commit = files_to_commit(wd_hierarchy, mt_hierarchy);

	println!("Files to commit {:?}", files_to_commit);	
}

fn files_to_commit(wd_hierarchy: HashMap<String, model::MetaData>, mt_hierarchy: model::Hierarchy) -> Vec<String>  {
	let mut files_to_commit: Vec<String> = Vec::new();

	for (filename, metadata) in wd_hierarchy.iter() {
		let actual_metadata = mt_hierarchy.get_latest_meta_data(&filename);
		println!("actual metadata {:?}", actual_metadata);

		match actual_metadata {
				Some(x) => (),
				None => files_to_commit.push(filename.clone())
			}
	    //files_to_commit.push(filename.clone());
	}


	files_to_commit
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
		pub fn get_latest_meta_data(&self, filename: &String) -> Option<&MetaData> {
			match self.files.get(filename) {
				Some(x) => x.get_last(),
				None => None
			}
		}
	}

	impl MetaDataSet {
		pub fn get_last(&self) -> Option<&MetaData> {
			self.metadata.last()
		}
	}

	impl MetaData {
		pub fn new_without_hash(timestamp: u64, size: u64) -> MetaData {
			MetaData {timestamp: timestamp, size: size, hash: "".to_string(), stored_hash: "".to_string()}
		}	
	}

}
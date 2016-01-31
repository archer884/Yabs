extern crate filetime;
extern crate rustc_serialize;
extern crate crypto;

use std::path::Path;
use std::collections::HashMap;


mod metadata;
mod workingdirectory;
mod store;

fn main() {
	let mut args = std::env::args();
	match args.nth(1) {
		Some(option) => dispatch_option(&option),
		None => println!("No option")
	}
}

fn dispatch_option(option: &str) {
	match option {
		"new" => new_repo(),
		"update" => update(),
		"commit" => commit(),
		_ => println!("Unknown option {}", option)
	}
}

fn new_repo() {
	println!("Creation of a new repo");
	let json_path = Path::new("metadata.json");
	metadata::create_emty_metadata_file(json_path);
}

fn update() {
	println!("Work in progress");
}

fn commit() {
	let data_path = Path::new("data");
	let json_path = Path::new("metadata.json");
	let store_path = Path::new("store");

	let wd_hierarchy : HashMap<String, model::MetaData> = workingdirectory::read_working_directory(data_path);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(json_path);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let files_to_commit = files_to_commit(wd_hierarchy, &mt_hierarchy);
	println!("{} files to commit", files_to_commit.len());

	let mut updated_metadata : HashMap<String, model::MetaData> = HashMap::new();
	for (filename, mut metadata) in files_to_commit {
		let hash = store::store_file(store_path, Path::new(&filename));
		metadata.add_hash(hash);

		updated_metadata.insert(filename, metadata);
	}	
	let updated_metadata = updated_metadata;

//	println!("Updated metadata {:?}", &updated_metadata);

	let mut mt_hierarchy = mt_hierarchy;
	mt_hierarchy.update(updated_metadata);

	metadata::write_metadata_file(json_path, mt_hierarchy);
}

fn files_to_commit(wd_hierarchy: HashMap<String, model::MetaData>, mt_hierarchy: &model::Hierarchy) -> HashMap<String, model::MetaData>  {
	let mut files_to_commit: HashMap<String, model::MetaData> = HashMap::new();

	for (filename, metadata) in wd_hierarchy.iter() {
		let actual_metadata = mt_hierarchy.get_latest_meta_data(&filename);
	//	println!("actual metadata {:?}", actual_metadata);

		match actual_metadata {
			Some(x) => {
				if(x.is_more_recent(&metadata)) {
					println!("- File to update {}", filename);
					files_to_commit.insert(filename.clone(), metadata.clone());	
				} else {
					println!("- No need to update {}", filename);
				}
			    ()},
			None => {
				println!("- New file {}", filename);
			    files_to_commit.insert(filename.clone(), metadata.clone());
			    () }
		}
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

	#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
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
		pub fn get_number_of_files(&self) -> usize {
			self.files.len()
		}	
		pub fn get_latest_meta_data(&self, filename: &String) -> Option<&MetaData> {
			match self.files.get(filename) {
				Some(x) => x.get_last(),
				None => None
			}
		}
		pub fn update(&mut self, newMetadata: HashMap<String, MetaData>) {
			self.nb_revision = self.nb_revision + 1;

			for (filename, metadata) in newMetadata {
				
				let new_metadata = self.new_metadata(&filename);

				if(new_metadata) {
					self.files.insert(filename, MetaDataSet::new_simple_meta_data_set(metadata));
				} else {
					let mut m = self.files.get_mut(&filename).unwrap();
					m.add_revision(metadata);
				}
			}
		}

		fn new_metadata(&self, filename: &String) -> bool {
			let actual_metadata = self.files.get(filename);

			match actual_metadata {
				Some(x) => false,
				None => true
			}
		}
	}

	impl MetaDataSet {
		pub fn get_last(&self) -> Option<&MetaData> {
			self.metadata.last()
		}
		pub fn new_simple_meta_data_set(m: MetaData) -> MetaDataSet {
			let mut v: Vec<MetaData> = Vec::new();
			v.push(m);
			MetaDataSet {metadata: v }
		}
		pub fn add_revision(&mut self, m: MetaData) {
			self.metadata.push(m);
		}
	}

	impl MetaData {
		pub fn new_without_hash(timestamp: u64, size: u64) -> MetaData {
			MetaData {timestamp: timestamp, size: size, hash: "".to_string(), stored_hash: "".to_string()}
		}
		pub fn add_hash(&mut self, hash: String) {
			self.hash = hash;
		}
		pub fn is_more_recent(&self, other: &MetaData) -> bool {
			self.timestamp < other.timestamp
		}	
	}

}
extern crate filetime;
extern crate rustc_serialize;
extern crate crypto;

use std::collections::HashMap;

mod metadata;
mod workingdirectory;
mod store;

use model::{Hierarchy, MetaData};

const DATA_PATH: &'static str = "data";
const JSON_PATH: &'static str = "metadata.json";
const STORE_PATH: &'static str = "store";

fn main() {
	match std::env::args().nth(1) {
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
	metadata::create_empty_metadata_file(JSON_PATH);
}

fn update() {
	let wd_hierarchy = workingdirectory::read_working_directory(DATA_PATH);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(JSON_PATH);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let file_top_update = files_to_update(wd_hierarchy, &mt_hierarchy);
	for (filename, metadata) in &file_top_update {
		store::extract_file(STORE_PATH, &metadata.get_hash(), DATA_PATH, filename, metadata.get_timestamp());
	}
}

fn files_to_update(wd_hierarchy: HashMap<String, MetaData>, mt_hierarchy: &Hierarchy) -> HashMap<String, MetaData>  {
	let mut file_to_update = HashMap::new();
	for (filename, metadataset) in mt_hierarchy.get_files() {
		let wd_metadata = wd_hierarchy.get(filename);
		match wd_metadata {
			Some(_) => println!("- Need to do something with {}", filename),
			None => {
				println!("- New file to update {}", filename);
			    file_to_update.insert(filename.clone(), metadataset.get_last().unwrap().clone());
		    }
		}
	}

	file_to_update
}


fn commit() {
	let wd_hierarchy  = workingdirectory::read_working_directory(DATA_PATH);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(JSON_PATH);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let files_to_commit = files_to_commit(wd_hierarchy, &mt_hierarchy);
	println!("{} files to commit", files_to_commit.len());

	let mut updated_metadata = HashMap::new();
	for (filename, mut metadata) in files_to_commit {
		let hash = store::store_file(STORE_PATH, &filename);
		metadata.add_hash(hash);
		updated_metadata.insert(filename, metadata);
	}
	let updated_metadata = updated_metadata;

	let mut mt_hierarchy = mt_hierarchy;
	mt_hierarchy.update(updated_metadata);

	metadata::write_metadata_file(JSON_PATH, mt_hierarchy);
}

fn files_to_commit(wd_hierarchy: HashMap<String, MetaData>, mt_hierarchy: &Hierarchy) -> HashMap<String, MetaData>  {
	let mut files_to_commit: HashMap<String, MetaData> = HashMap::new();

	for (filename, metadata) in wd_hierarchy.iter() {
		let actual_metadata = mt_hierarchy.get_latest_meta_data(&filename);
		match actual_metadata {
			Some(x) => {
				if x.is_more_recent(&metadata) {
					println!("- File to update {}", filename);
					files_to_commit.insert(filename.clone(), metadata.clone());
				} else {
					println!("- No need to update {}", filename);
				}
			},
			None => {
				println!("- New file {}", filename);
			    files_to_commit.insert(filename.clone(), metadata.clone());
			}
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
			Hierarchy {
				nb_revision: 1,
				files: HashMap::new(),
			}
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

		pub fn update(&mut self, new_metadata: HashMap<String, MetaData>) {
			self.nb_revision = self.nb_revision + 1;

			for (filename, metadata) in new_metadata {
				let new_metadata = self.new_metadata(&filename);

				if new_metadata {
					self.files.insert(filename, MetaDataSet::new_simple_meta_data_set(metadata));
				} else {
					let mut m = self.files.get_mut(&filename).unwrap();
					m.add_revision(metadata);
				}
			}
		}

		fn new_metadata(&self, filename: &String) -> bool {
			!self.files.get(filename).is_some()
		}

		pub fn get_files(&self) -> &HashMap<String, MetaDataSet> {
			&self.files
		}
	}

	impl MetaDataSet {
		pub fn get_last(&self) -> Option<&MetaData> {
			self.metadata.last()
		}

		pub fn new_simple_meta_data_set(m: MetaData) -> MetaDataSet {
			let mut v: Vec<MetaData> = Vec::new();
			v.push(m);
			MetaDataSet {
				metadata: v
			}
		}

		pub fn add_revision(&mut self, m: MetaData) {
			self.metadata.push(m);
		}
	}

	impl MetaData {
		pub fn new_without_hash(timestamp: u64, size: u64) -> MetaData {
			MetaData {
				timestamp: timestamp,
				size: size,
				hash: String::new(),
				stored_hash: String::new(),
			}
		}

		pub fn add_hash(&mut self, hash: String) {
			self.hash = hash;
		}

		pub fn is_more_recent(&self, other: &MetaData) -> bool {
			self.timestamp < other.timestamp
		}

		pub fn get_timestamp(&self) -> u64 {
			self.timestamp
		}

		pub fn get_hash(&self) -> String {
			self.hash.clone()
		}
	}
}

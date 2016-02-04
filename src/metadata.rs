use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use rustc_serialize::json;

use model::Hierarchy;

#[allow(unused)]
pub fn create_empty_metadata_file<P: AsRef<Path>>(path: P) {
	println!("Creating empty metadata file in {}", path.as_ref().to_str().unwrap());

	let hierarchy = Hierarchy::new_empty();
	let json_hierarchy = json::encode(&hierarchy).unwrap();

	let mut file = File::create(&path).unwrap();
    let bytes = json_hierarchy.into_bytes();

    file.write_all(&bytes);
    file.sync_all();
}

#[allow(unused)]
pub fn read_metadata_file<P: AsRef<Path>>(path: P) -> Hierarchy {
	let mut file = File::open(path).unwrap();
	let mut json = String::new();

	file.read_to_string(&mut json);

    json::decode(&json).unwrap()
}

#[allow(unused)]
pub fn write_metadata_file<P: AsRef<Path>>(path: P, hierarchy: Hierarchy) {
	let json_hierarchy = json::encode(&hierarchy).unwrap();
    let byte = json_hierarchy.into_bytes();

	let mut file = File::create(path).unwrap();

    file.write_all(&byte);
    file.sync_all();
}

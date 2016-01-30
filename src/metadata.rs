
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use filetime::FileTime;
use rustc_serialize::json;

use model::Hierarchy;
use model::MetaDataSet;

pub fn create_emty_metadata_file<P: AsRef<Path>>(path: P) {
	println!("Creating empty metadata file");  

	let hierarchy = Hierarchy::new_empty();

	println!("hey : {:?}", hierarchy);

	let json_hierarchy =   json::encode(&hierarchy).unwrap();

	println!("hey in json: {:?}", json_hierarchy);    

	let mut file = File::create(&path).unwrap();

    let u8_vec = json_hierarchy.into_bytes();
	let u8_slice = &u8_vec[..];
    file.write_all(u8_slice);

    file.sync_all();
}



use std::collections::HashMap;
use filetime::FileTime;
use std::path::Path;
use std::fs;

use model::MetaData;

pub fn read_working_directory<P: AsRef<Path>>(root: P) -> HashMap<String, MetaData> {
	let mut hierarchy = HashMap::new();
	read_folder_rec(root, &mut hierarchy);
	hierarchy
}

fn read_folder_rec<P: AsRef<Path>>(root: P, hierarchy: &mut HashMap<String, MetaData>) {
	let paths = fs::read_dir(root).unwrap();
	for entry in paths.filter_map(|entry| entry.ok()) {
		if let Ok(info) = entry.metadata() {
			if info.is_dir() {
				read_folder_rec(&entry.path(), hierarchy);
			} else if info.is_file() {
				let path_str = entry.path().to_str().expect("cannot convert path to utf8 string").to_string();
				let last_change = FileTime::from_last_modification_time(&info).seconds_relative_to_1970();
				let metadata = MetaData::new_without_hash(last_change, info.len());

				hierarchy.insert(path_str, metadata);
			}
		}
    }
}

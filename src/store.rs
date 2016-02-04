use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use filetime::FileTime;
use filetime;

use std::io::Read;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[allow(unused)]
pub fn store_file<P1, P2>(store_path: P1, source_file: P2) -> String
	where P1: AsRef<Path>,
		  P2: AsRef<Path>,
{
	let tmp_path = store_path.as_ref().join("tmp");
	fs::copy(&source_file, &tmp_path);

	let hash = hash(&source_file);
	let final_path = store_path.as_ref().join(&hash);

	fs::rename(&tmp_path, &final_path);
	hash
}

#[allow(unused)]
pub fn extract_file<P1, P2>(store_path: P1, hash: &str, data_path: P2, filename: &str, timestamp: u64)
	where P1: AsRef<Path>,
		  P2: AsRef<Path>,
{
	let file_in_store = store_path.as_ref().join(hash);
	let tmp_path = data_path.as_ref().join("tmp");

	println!("Extract from {} to {} ", file_in_store.to_str().unwrap(), filename);

	fs::copy(&file_in_store, &tmp_path);
	fs::rename(&tmp_path, filename);

	let seconds_since_1970 = FileTime::from_seconds_since_1970(timestamp, 0);
	filetime::set_file_times(filename, seconds_since_1970, seconds_since_1970);
}

fn hash<P: AsRef<Path>>(file: P) -> String {
	let f = File::open(file).unwrap();
	let mut reader = BufReader::new(f);
	let mut buffer = [0; 512];

	let mut hasher = Sha256::new();
	while let Ok(bytes_read) = reader.read(&mut buffer) {
		if bytes_read == 0 { break; }
		hasher.input(&buffer[0..bytes_read]);
	}
	hasher.result_str()
}

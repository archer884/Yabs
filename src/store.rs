use std::path;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;

use std::io::Read;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn store_file(store_path: &Path, source_file: &Path) -> String {
	let tmp_path = store_path.join("tmp");

	fs::copy(&source_file, &tmp_path);

	let hash = hash(&source_file);

	let final_path = store_path.join(&hash);

	fs::rename(&tmp_path, &final_path);

	hash
}

fn hash(file: &Path) -> String {
	let mut f = File::open(&file).unwrap();
	let mut reader = BufReader::new(f);
	let mut buffer = [0; 512];

	let mut hasher = Sha256::new();

	loop {
		let bytesRead = reader.read(&mut buffer).unwrap();
		if bytesRead == 512 {
			hasher.input(&buffer);
		} else {
			let v : Vec<u8> = buffer.iter().cloned().collect();
			let (x, y) = v.split_at(bytesRead);
			hasher.input(x);
		}
		if bytesRead == 0 { break; }
	}

	let hex = hasher.result_str();
//	println!("Hash: {:?}", hex);
	hex
}

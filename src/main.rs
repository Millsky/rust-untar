extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use std::env;
use std::io;
use std::fs::File;
use std::fs::{self};
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    // Get the path from the console
    let path = &args[1];
    // Get the output_path from the console
    let output_path = &args[2];
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let unpack_path: &str = &String::from(format!("./{}", output_path));
    archive.unpack(unpack_path).unwrap();
    let mut entries = fs::read_dir(unpack_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();
    println!("{:?}", entries);
    // The entries have now been sorted by their path.
    Ok(())
}

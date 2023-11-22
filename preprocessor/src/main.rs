//! Build script for TTL2 maps.

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
};

/// Relative path to the map scripts.
const SCRIPTS: &str = "../scripts";

/// Absolute path to the random map folder directory.
const RMS_DIR: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/AoE2DE/resources/_common/random-map-scripts";

/// Absolute path to the TC mod directory.
const TC_DIR: &str = "C:/Users/twest/Games/Age of Empires 2 DE/76561198003545293/mods/local/TCMapsTest/resources/_common/random-map-scripts";

/// Runs preprocessing steps and writes the output to the test mod.
/// Overwrites all files in the test mod.
/// Does not remove unnecessary files from the test mod.
fn main() -> std::io::Result<()> {
    // Supports running from different locations.
    let paths = match fs::read_dir(SCRIPTS) {
        Ok(ps) => ps,
        Err(_) => fs::read_dir("scripts").unwrap(),
    };

    let mut directories = vec![paths];
    while let Some(dir) = directories.pop() {
        for path in dir {
            let path = path?;

            let src_path = path.path();
            if src_path.is_dir() {
                directories.push(fs::read_dir(src_path)?);
                continue;
            }
            let src_file = File::open(src_path)?;
            let mut src_reader = BufReader::new(src_file);

            let map_name = path.file_name();
            let str_name = map_name.to_str().unwrap();
            let out_path = if str_name.starts_with("TC") {
                TC_DIR
            } else {
                RMS_DIR
            };
            let dest_path = Path::new(out_path).join(map_name);
            let dest_file = File::create(dest_path)?;
            let mut dest_writer = BufWriter::new(dest_file);
            preprocessor::process_script(&mut src_reader, &mut dest_writer)?;
        }
    }
    Ok(())
}

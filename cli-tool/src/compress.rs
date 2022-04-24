extern crate zip;

use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use zip::write::FileOptions;

use walkdir::DirEntry;
use std::path::Path;
use std::fs::File;

// Copied example from https://github.com/zip-rs/zip/blob/5d0f198124946b7be4e5969719a7f29f363118cd/examples/write_dir.rs
pub async fn compress<T>(it: &mut dyn Iterator<Item=DirEntry>, prefix: &str, writer: T)
              -> zip::result::ZipResult<()>
    where T: Write+Seek
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            log::info!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        }
        else if !name.as_os_str().is_empty() {
            log::info!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}


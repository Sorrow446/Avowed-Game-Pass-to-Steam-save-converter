mod structs;
mod utils;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use uuid::Uuid;
use clap::Parser;

use crate::structs::{Args, Config};

const FNAME_OFFSETS: [u64; 3] = [136, 296, 456];

const START_DATA: &[u8; 61] = &[
    0x03, 0x00, 0xFE, 0x5A, 0x03, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4D, 0x65, 0x74, 0x61,
    0x44, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x53, 0x61, 0x76,
    0x65, 0x47, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x53, 0x63,
    0x72, 0x65, 0x65, 0x6E, 0x73, 0x68, 0x6F, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00
];

fn find_container_file(dir: &PathBuf) -> Result<Option<String>, Box<dyn Error>>  {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name()
            .ok_or("failed to get filename of path")?
            .to_string_lossy();

        if file_name.starts_with("container.") {
            return Ok(Some(file_name.into_owned()));
        }
    }

    Ok(None)
}

fn read_guid(f: &mut File) -> Result<String, Box<dyn Error>> {
    let mut buf = [0u8; 16];
    f.read_exact(&mut buf)?;
    let guid = Uuid::from_bytes_le(buf);

    Ok(guid.simple().to_string().to_uppercase())
}

fn read_container_file(container_path: &PathBuf, base_path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut f = File::open(container_path)?;
    let mut paths: Vec<PathBuf> = Vec::new();

    let file_size = f.metadata()?.len();
    if file_size != 488 {
        return Err("bad container file size".into());
    }

    for offset in FNAME_OFFSETS {
        f.seek(SeekFrom::Start(offset))?;

        let guid = read_guid(&mut f)?;

        let path = base_path.join(guid);
        paths.push(path);
    }

    Ok(paths)
}

fn parse_config() -> Result<Config, Box<dyn Error>> {
    let args = Args::parse();

    let folder_name = args.in_path.file_name()
        .ok_or("no folder name")?
        .to_string_lossy()
        .into_owned();

    let exe_path = utils::get_exe_path()?;
    let out_path = args.out_path.unwrap_or(exe_path);
    let joined_out_path = out_path.join(folder_name);
    fs::create_dir_all(&out_path)?;

    let final_out_path = utils::append_to_path_buf(&joined_out_path, ".sav");

    let config = Config {
        in_path: args.in_path,
        out_path: final_out_path,
    };

    Ok(config)
}

fn write_save(path: &PathBuf, paths: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(path)?;
    f.write_all(START_DATA)?;

    let metadata_data = fs::read(&paths[0])?;
    f.write_all(&metadata_data)?;

    let upk_data = fs::read(&paths[1])?;
    f.write_all(&upk_data)?;

    let thumb_data = fs::read(&paths[2])?;
    f.write_all(&thumb_data)?;

    f.seek(SeekFrom::Start(21))?;
    let metadata_size_u32 = metadata_data.len() as u32;
    let metadata_size_bytes: &[u8; 4] = &metadata_size_u32.to_le_bytes();
    f.write_all(metadata_size_bytes)?;

    f.seek(SeekFrom::Current(13))?;
    let upk_chunk_size_u32 = upk_data.len() as u32;
    let upk_chunk_size_bytes: &[u8; 4] = &upk_chunk_size_u32.to_le_bytes();
    f.write_all(upk_chunk_size_bytes)?;

    f.seek(SeekFrom::Current(15))?;
    let thumb_size_u32 = thumb_data.len() as u32;
    let thumb_size_bytes: &[u8; 4] = &thumb_size_u32.to_le_bytes();
    f.write_all(thumb_size_bytes)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config()?;
    let save_path = config.in_path;

    let container_fname = match find_container_file(&save_path) {
        Ok(Some(fname)) => fname,
        Ok(None) => return Err("container file not present in save folder".into()),
        Err(e) => return Err(e)
    };

    let container_path = save_path.join(container_fname);

    println!("Reading container file...");
    let paths = read_container_file(&container_path, &save_path)?;

    println!("Creating new save...");
    write_save(&config.out_path, &paths)?;

    println!("-> {}", config.out_path.to_string_lossy());
    Ok(())
}
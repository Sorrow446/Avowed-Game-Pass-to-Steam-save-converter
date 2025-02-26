use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn get_exe_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let parent_dir = exe_path.parent()
        .ok_or("failed to get path of executable")?;
    let exe_path_buf = PathBuf::from(parent_dir);
    Ok(exe_path_buf)
}

pub fn append_to_path_buf(path: &PathBuf, to_append: &str) -> PathBuf {
    let path_str = path.to_string_lossy();
    let new_path_str = format!("{}{}", path_str, to_append);
    PathBuf::from(new_path_str)
}
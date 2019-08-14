use std::fs::{self, DirEntry};

use quick_error::ResultExt;

use crate::error::SageError;



pub fn get_files_list(path: &String) -> Result<Vec<DirEntry>, SageError> {
    let files: Vec<_> = fs::read_dir(path).context(path)?
        .filter_map(Result::ok)
        .collect();

    Ok(files)
}

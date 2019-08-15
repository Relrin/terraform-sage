use std::fs::{self, DirEntry};

use quick_error::ResultExt;

use crate::error::SageError;


pub fn get_files_list(path: &String) -> Result<Vec<DirEntry>, SageError> {
    let files: Vec<_> = fs::read_dir(path).context(path)?
        .filter_map(Result::ok)
        .collect();

    Ok(files)
}


#[cfg(test)]
mod tests {
    use crate::utils::get_files_list;

    #[test]
    fn test_get_files_list_returns_vector_of_entries() {
        let path = String::from("./examples/approach_two");
        let result = get_files_list(&path);

        assert_eq!(result.is_ok(), true);
        let list = result.unwrap();
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_get_files_list_returns_error_for_non_existing_directory() {
        let path = String::from("./NOT_EXISTING_DIR/");
        let result = get_files_list(&path);

        assert_eq!(result.is_err(), true);
    }
}

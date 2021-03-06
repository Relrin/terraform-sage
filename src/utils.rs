use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::Path;

use quick_error::ResultExt;

use crate::error::SageError;

pub const CONFIG_DIRECTORY_NAME: &'static str = "configs";

// Returns file extension.
pub fn get_extension_from_filename(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .map_or("", |ext| OsStr::to_str(ext).unwrap_or(""))
}

// Returns list of files for directory specified in `path` parameter.
pub fn get_files_list(path: &String) -> Result<Vec<DirEntry>, SageError> {
    let files: Vec<_> = fs::read_dir(path)
        .context(path)?
        .filter_map(Result::ok)
        .collect();

    Ok(files)
}

// Returns dictionary, where the key is configuration name and value is the
// full path to this directory. Search is happening by directory specified
// in `path` parameter.
pub fn get_configs(path: &String) -> Result<HashMap<String, String>, SageError> {
    let configs = get_files_list(path)?
        .into_iter()
        .filter(|f| match f.metadata() {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        })
        .filter(|f| f.file_name() == CONFIG_DIRECTORY_NAME)
        .flat_map(|f| {
            let dir = f.path().to_string_lossy().into_owned();
            match get_files_list(&dir) {
                Ok(vec) => vec,
                Err(_) => vec![],
            }
        })
        .filter(|f| match f.metadata() {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        })
        .map(|dir| {
            (
                dir.file_name().to_string_lossy().into_owned(), // directory name
                dir.path().to_string_lossy().into_owned(),      // path
            )
        })
        .collect();

    Ok(configs)
}

// Checks that the given name is represented in the configurations list.
pub fn is_correct_config(name: &String, configs: HashMap<String, String>) -> Result<(), SageError> {
    match configs.contains_key(name) {
        true => Ok(()),
        false => {
            let message = String::from(format!("Configuration with {} name was not found.", name));
            Err(SageError::InvalidConfig(message))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_configs, get_files_list, is_correct_config};

    #[test]
    fn test_get_files_list_returns_vector_of_entries() {
        let path = String::from("./examples/approach_two");
        let result = get_files_list(&path);

        assert_eq!(result.is_ok(), true);
        let list = result.unwrap();
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_get_files_list_returns_error_for_invalid_path() {
        let path = String::from("./NOT_EXISTING_DIR/");
        let result = get_files_list(&path);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_get_configs_returns_all_available_configurations() {
        let path = String::from("./examples/approach_two");
        let result = get_configs(&path);

        assert_eq!(result.is_ok(), true);
        let configs = result.unwrap();
        assert_eq!(configs.len(), 3);
        assert_eq!(configs.contains_key("dev"), true);
        assert_eq!(configs.contains_key("staging"), true);
        assert_eq!(configs.contains_key("production"), true);
    }

    #[test]
    fn test_get_configs_returns_empty_hashmap() {
        let path = String::from(".");
        let result = get_configs(&path);

        assert_eq!(result.is_ok(), true);
        let configs = result.unwrap();
        assert_eq!(configs.len(), 0);
    }

    #[test]
    fn test_get_configs_returns_error_for_invalid_path() {
        let path = String::from("./NOT_EXISTING_DIR/");
        let result = get_configs(&path);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_is_correct_config_return_empty_result() {
        let path = String::from("./examples/approach_two");
        let configs = get_configs(&path).unwrap();
        let result = is_correct_config(&"dev".to_owned(), configs);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_is_correct_config_returns_error() {
        let path = String::from("./examples/approach_two");
        let configs = get_configs(&path).unwrap();
        let result = is_correct_config(&"INVALID".to_owned(), configs);

        assert_eq!(result.is_err(), true);
    }
}

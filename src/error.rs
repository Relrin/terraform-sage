use std::io::{self};

use quick_error::quick_error;


quick_error! {
    #[derive(Debug)]
    pub enum SageError {
        Io(err: io::Error, path: String) {
            display("I/O error with {:?}: {}", path, err)
            context(path: &'a String, err: io::Error)
                -> (err, path.to_string())
        }
    }
}

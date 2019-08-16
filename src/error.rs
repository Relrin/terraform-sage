use std::io::{self};

use handlebars::TemplateRenderError;
use quick_error::quick_error;


quick_error! {
    #[derive(Debug)]
    pub enum SageError {
        Io(err: io::Error, path: String) {
            display("I/O error with {:?}: {}", path, err)
            context(path: &'a String, err: io::Error)
                -> (err, path.to_string())
        }
        InvalidConfig(message: String) {
            display("Invalid configuration: {}", message)
        }
        TemplateRender(err: TemplateRenderError, filename: String) {
            display("Template rendering error for {} file: {}", filename, err)
            context(filename: &'a String, err: TemplateRenderError)
                -> (err, filename.to_string())
        }
    }
}

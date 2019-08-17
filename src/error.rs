use std::io;

use handlebars::TemplateRenderError;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum SageError {
        Io(err: io::Error, source: String) {
            display("I/O error with {}: {}", source, err)
            context(source: &'a String, err: io::Error)
                -> (err, source.to_string())
        }
        InvalidConfig(message: String) {
            display("Invalid configuration: {}", message)
        }
        TemplateRender(err: TemplateRenderError, filename: String) {
            display("Template rendering error for {} file: {}", filename, err)
            context(filename: &'a String, err: TemplateRenderError)
                -> (err, filename.to_string())
        }
        TerraformError(err: io::Error, command: String) {
            display("Terraform error for `{}` command: {}", command, err)
            context(command: &'a str, err: io::Error)
                -> (err, command.to_owned())
        }
    }
}

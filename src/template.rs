use std::fs;
use std::io::Write;

use handlebars::Handlebars;
use quick_error::ResultExt;
use serde_json::json;

use crate::error::SageError;
use crate::terminal::print_info;

pub const CONFIG_TEMPLATE_PARAM: &'static str = "CONFIG_NAME";

// Generates file name for Terraform main module.
pub fn generate_file_name(target: &String) -> String {
    format!("main-{}.tf", target).to_owned()
}

// Generates new Terraform module from the file with name specified
// in `target` parameter and save the rendered content in file with
// the name specified in `out` parameter.
pub fn generate_from_template(
    handlebars: &Handlebars,
    config: &String,
    target: &String,
    out: &String,
) -> Result<String, SageError> {
    let template = fs::read_to_string(target).context(target)?;

    print_info("Generating Terraform file...");
    let template_parameters = json!({ CONFIG_TEMPLATE_PARAM: config });
    let module = handlebars
        .render_template(&template, &template_parameters)
        .context(out)?;

    let mut file = fs::File::create(out).context(out)?;
    file.write_all(module.as_bytes()).context(out)?;
    print_info(&format!("New Terraform file was created by path: {}", out));
    Ok(out.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use std::path::Path;

    use handlebars::Handlebars;

    use crate::template::generate_from_template;

    #[test]
    fn test_generate_from_template() {
        let handlebars = Handlebars::new();
        let config = String::from("dev");
        let used_directory = Path::new("./examples/approach_two");
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("main.tpl")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&handlebars, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_ok(), true);
        fs::remove_file(path_to_out).unwrap();
    }

    #[test]
    fn test_generate_from_template_returns_error_for_invalid_path() {
        let handlebars = Handlebars::new();
        let config = String::from("dev");
        let used_directory = Path::new("./examples/INVALID_PATH");
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("main.tpl")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&handlebars, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_generate_from_template_returns_error_for_invalid_target_file_name() {
        let handlebars = Handlebars::new();
        let config = String::from("dev");
        let used_directory = Path::new("./examples/examples");
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("INVALID_FILE_NAME")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&handlebars, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_err(), true);
    }
}

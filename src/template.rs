use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use handlebars::Handlebars;
use quick_error::ResultExt;
use serde_json::json;
use toml::Value as TomlValue;

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
    directory: &String,
    config: &String,
    target: &String,
    out: &String,
) -> Result<String, SageError> {
    let handlebars = Handlebars::new();
    let template = fs::read_to_string(target).context(target)?;
    let mut context = get_template_context(directory, config);
    context.insert(CONFIG_TEMPLATE_PARAM.to_string(), config.clone());

    print_info("Generating Terraform file...");
    let template_parameters = json!(context);
    let module = handlebars
        .render_template(&template, &template_parameters)
        .context(out)?;

    let mut file = fs::File::create(out).context(out)?;
    file.write_all(module.as_bytes()).context(out)?;
    print_info(&format!("New Terraform file was created by path: {}", out));
    Ok(out.to_string())
}

// Parses the template context from the properties.toml file, located
// in the given directory with the `config` name. If the file doesn't
// exist or the context not found, then returns an empty context.
pub fn get_template_context(directory: &String, config: &String) -> HashMap<String, String>{
    let mut context = HashMap::new();

    let toml_path: String = Path::new(directory)
        .join("configs/context.toml")
        .to_string_lossy()
        .into_owned();
    let raw_data = fs::read_to_string(&toml_path).unwrap_or("".to_string());

    match raw_data.parse::<TomlValue>() {
        Ok(toml_root) => {
            match toml_root.as_table() {
                Some(table) => table
                    .iter()
                    .filter(|(key, _value)| key == &config)
                    .for_each(|(_key, value)| {
                        let variables = convert_toml_to_hashmap(value);
                        context.extend(variables);
                    }),
                None => return context,
            }
        },
        Err(_) => return context,
    };

    context
}

// Converts the given sub-toml into the flat hashmap that stores key-value pairs
// for the used environment. Supported only the strings as the values.
fn convert_toml_to_hashmap(value: &TomlValue) -> HashMap<String, String> {
    let mut context = HashMap::new();

    if let Some(definitions) = value.as_table() {
        definitions
            .iter()
            .for_each(|(key, toml_value)| {
                match toml_value {
                    TomlValue::String(value) => {
                        context.insert(key.clone(), value.clone());
                    },
                    _ => {},
                };
            })
    };

    context
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use std::path::Path;

    use crate::template::generate_from_template;

    #[test]
    fn test_generate_from_template() {
        let directory = "./examples/approach_two".to_string();
        let config = String::from("dev");
        let used_directory = Path::new(directory.as_str());
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("main.tpl")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&directory, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_ok(), true);
        fs::remove_file(path_to_out).unwrap();
    }

    #[test]
    fn test_generate_from_template_returns_error_for_invalid_path() {
        let directory = "./examples/INVALID_PATH".to_string();
        let config = String::from("dev");
        let used_directory = Path::new(directory.as_str());
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("main.tpl")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&directory, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_generate_from_template_returns_error_for_invalid_target_file_name() {
        let directory = "./example/examples".to_string();
        let config = String::from("dev");
        let used_directory = Path::new(directory.as_str());
        let out_directory = env::temp_dir();
        let path_to_target = used_directory
            .join("INVALID_FILE_NAME")
            .to_string_lossy()
            .into_owned();
        let path_to_out = out_directory
            .join("main.tf")
            .to_string_lossy()
            .into_owned();
        let result = generate_from_template(&directory, &config, &path_to_target, &path_to_out);

        assert_eq!(result.is_err(), true);
    }
}

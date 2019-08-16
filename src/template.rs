use std::fs;
use std::io::Write;

use handlebars::Handlebars;
use serde_json::json;
use quick_error::ResultExt;

use crate::error::SageError;
use crate::terminal::print_info;


pub fn generate_from_template(handlebars: &Handlebars, config: &String, target: &String, out: &String) -> Result<fs::File, SageError> {
    let template = fs::read_to_string(target).context(target)?;

    print_info("Generating Terraform file...");
    let template_parameters = json!({"CONFIG_NAME": config});
    let module = handlebars.render_template(&template, &template_parameters).context(out)?;

    let mut file = fs::File::create(out).context(out)?;
    file.write_all(module.as_bytes()).context(out)?;
    print_info(&format!("New Terraform file was created by path: {}", out));
    Ok(file)
}

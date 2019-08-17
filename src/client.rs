use std::path::Path;

use crate::cli::Command;
use crate::error::SageError;
use crate::template::{generate_file_name, generate_from_template};
use crate::terminal::{print_command_done, print_error, print_info, print_warning};
use crate::terraform::{
    delete_terraform_file, get_terraform_init_args, terraform_call_without_input,
};
use crate::utils::{get_configs, is_correct_config};

use handlebars::Handlebars;

pub struct Client {
    handlebars: Handlebars,
}

impl Client {
    pub fn new() -> Client {
        Client {
            handlebars: Handlebars::new(),
        }
    }

    pub fn run(&self, command: &Command) {
        let result = match command {
            Command::Init {
                config,
                directory,
                target,
                template,
                out,
                cleanup,
                extra,
            } => self.init_terraform(config, directory, target, template, out, *cleanup, extra),
            Command::List { directory } => self.show_configurations(directory),
            Command::Generate {
                directory,
                config,
                template,
                out,
            } => match self.generate_main_tf(directory, config, template, out) {
                Ok(_out_filepath) => Ok(()),
                Err(err) => Err(err),
            },
        };

        match result {
            Ok(_) => print_command_done(),
            Err(e) => print_error(e),
        }
    }

    // Initializes working directory with Terraform files.
    //
    // By default tries to generate new main.tf module with the `out` name and
    // saves it in `directory` path. The template module are looking in `directory`
    // path with the `template` name.
    //
    // If the module already created by someone, you can specify this module
    // via usage the `target` option. This option must contain path to this file.
    //
    // For deleting the template-based files with the name specified in `out`
    // option and saved by `directory` path, just specify the --cleanup option
    // before executing init command.
    //
    fn init_terraform(
        &self,
        config: &String,
        directory: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
        cleanup: bool,
        extra: &Vec<String>,
    ) -> Result<(), SageError> {
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let terraform_args = get_terraform_init_args(directory, extra);
        terraform_call_without_input("init", &terraform_args)?;

        if cleanup {
            delete_terraform_file(&main_filepath)?;
        };
        Ok(())
    }

    fn get_main_tf(
        &self,
        directory: &String,
        config: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
    ) -> Result<String, SageError> {
        match target {
            Some(file_name) => Ok(file_name.to_owned()),
            None => {
                print_warning("The `target` option was not specified.");
                let out_file_name = out.clone().unwrap_or(generate_file_name(config));
                self.generate_main_tf(directory, config, template, &out_file_name)
            }
        }
    }

    fn show_configurations(&self, directory: &String) -> Result<(), SageError> {
        let configs = get_configs(directory)?;

        match configs.len() {
            count if count > 0 => {
                print_info("Available configurations:");
                configs
                    .keys()
                    .into_iter()
                    .for_each(|config| print_info(&format!("- {}", config)));
            }
            _ => print_warning("Configurations were not found."),
        };

        Ok(())
    }

    fn generate_main_tf(
        &self,
        directory: &String,
        config: &String,
        template: &String,
        out: &String,
    ) -> Result<String, SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs)?;
        let used_directory = Path::new(directory);
        let path_to_target = used_directory.join(template).to_string_lossy().into_owned();
        let path_to_out = used_directory.join(out).to_string_lossy().into_owned();
        generate_from_template(&self.handlebars, config, &path_to_target, &path_to_out)
    }
}

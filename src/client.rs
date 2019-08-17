use std::fs::File;
use std::path::Path;

use crate::cli::Command;
use crate::error::SageError;
use crate::template::generate_from_template;
use crate::terminal::{print_command_done, print_error, print_info, print_warning};
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
            Command::List { directory } => self.show_configurations(directory),
            Command::Generate {
                directory,
                config,
                target,
                out,
            } => match self.generate_main_tf(directory, config, target, out) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
        };

        match result {
            Ok(_) => print_command_done(),
            Err(e) => print_error(e),
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
        target: &String,
        out: &String,
    ) -> Result<File, SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs)?;
        let used_directory = Path::new(directory);
        let path_to_target = used_directory.join(target).to_string_lossy().into_owned();
        let path_to_out = used_directory.join(out).to_string_lossy().into_owned();
        generate_from_template(&self.handlebars, config, &path_to_target, &path_to_out)
    }
}

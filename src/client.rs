use std::path::Path;

use crate::cli::Command;
use crate::error::SageError;
use crate::template::{generate_file_name, generate_from_template};
use crate::terminal::{print_command_done, print_error, print_info, print_warning};
use crate::terraform::TerraformClient;
use crate::utils::{get_configs, is_correct_config};

pub struct SageClient {
    terraform: TerraformClient,
}

impl SageClient {
    // Initialize a new instance of Sage client.
    pub fn new() -> Self {
        SageClient {
            terraform: TerraformClient::new(),
        }
    }

    // An entry point for Terraform-Sage client.
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
            Command::Plan {
                config,
                directory,
                target,
                template,
                out,
                cleanup,
                extra,
            } => self.generate_plan_execution(config, directory, target, template, out, *cleanup, extra),
            Command::Apply {
                config,
                directory,
                target,
                template,
                out,
                cleanup,
                extra,
            } => self.apply_changes(config, directory, target, template, out, *cleanup, extra),
            Command::Destroy {
                config,
                directory,
                target,
                template,
                out,
                cleanup,
                extra,
            } => self.destroy_infrastructure(config, directory, target, template, out, *cleanup, extra),
            Command::Output {
                config,
                directory,
                target,
                template,
                out,
                cleanup,
                extra,
            } => self.output_variables(config, directory, target, template, out, *cleanup, extra),
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
        let configs = get_configs(directory)?;
        is_correct_config(config, configs)?;
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let terraform_args = self.terraform.get_init_args(directory, extra);
        self.terraform.call_without_input("init", &terraform_args)?;

        if cleanup {
            self.terraform.delete_main_tf(&main_filepath)?;
        };
        Ok(())
    }

    // Generates an execution plan for Terraform.
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
    fn generate_plan_execution(
        &self,
        config: &String,
        directory: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
        cleanup: bool,
        extra: &Vec<String>,
    ) -> Result<(), SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs.clone())?;
        let configs_copy = configs.clone();
        let configs_path = configs_copy.get(config).unwrap();
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let terraform_args = self.terraform.get_command_args(configs_path, directory, extra);
        self.terraform.call_without_input("plan", &terraform_args)?;

        if cleanup {
            self.terraform.delete_main_tf(&main_filepath)?;
        };
        Ok(())
    }

    // Creates or updates infrastructure in according to Terraform configuration.
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
    fn apply_changes(
       &self,
        config: &String,
        directory: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
        cleanup: bool,
        extra: &Vec<String>,
    ) -> Result<(), SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs.clone())?;
        let configs_copy = configs.clone();
        let configs_path = configs_copy.get(config).unwrap();
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let terraform_args = self.terraform.get_command_args(configs_path, directory, extra);
        self.terraform.call_with_input("apply", &terraform_args)?;

        if cleanup {
            self.terraform.delete_main_tf(&main_filepath)?;
        };
        Ok(())
    }

    // Deletes all resources in infrastructure managed by Terraform.
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
    fn destroy_infrastructure(
        &self,
        config: &String,
        directory: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
        cleanup: bool,
        extra: &Vec<String>,
    ) -> Result<(), SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs.clone())?;
        let configs_copy = configs.clone();
        let configs_path = configs_copy.get(config).unwrap();
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let terraform_args = self.terraform.get_command_args(configs_path, directory, extra);
        self.terraform.call_with_input("destroy", &terraform_args)?;

        if cleanup {
            self.terraform.delete_main_tf(&main_filepath)?;
        };
        Ok(())
    }

    // Reads output variables from a Terraform state file and prints them.
    //
    // If `target` option contains path to *.tf module, then it will be
    // returned to the caller.
    //
    // Otherwise, this method will use the specified name for output file in
    // `out` parameter or generate a new file name, then extract the base
    // template by path specified in `template` parameter, call a template
    // renderer and save it by directory, specified in `directory` parameter.
    fn output_variables(
        &self,
        config: &String,
        directory: &String,
        target: &Option<String>,
        template: &String,
        out: &Option<String>,
        cleanup: bool,
        extra: &Vec<String>,
    ) -> Result<(), SageError> {
        let configs = get_configs(directory)?;
        is_correct_config(config, configs.clone())?;
        let out_filename = Some(out.clone().unwrap_or(String::from("main.tf")));
        let main_filepath = self.get_main_tf(directory, config, target, template, &out_filename)?;
        let mut terraform_args = Vec::new();
        terraform_args.extend(extra.iter().cloned());
        self.terraform.call_without_input("output", &terraform_args)?;

        if cleanup {
            self.terraform.delete_main_tf(&main_filepath)?;
        };
        Ok(())
    }

    // Returns a path to the used Terraform main.tf module.
    //
    // If `target` option contains path to *.tf module, then it will be
    // returned to the caller.
    //
    // Otherwise, this method will use the specified name for output file in
    // `out` parameter or generate a new file name, then extract the base
    // template by path specified in `template` parameter, call a template
    // renderer and save it by directory, specified in `directory` parameter.
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

    // Prints all available configurations, stored by path in `directory` parameter.
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

    // Generates a new Terraform main module.
    //
    // It used the directory, specified in `directory` parameter as the main
    // working directory, generates file pathes to target/out files, extracts
    // the template from the file with `target` name and put the rendered text
    // in file with `out` name.
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
        generate_from_template(directory, config, &path_to_target, &path_to_out)
    }
}

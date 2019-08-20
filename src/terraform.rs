use std::fs;
use std::io::Read;
use std::process::{Command, Stdio};

use quick_error::ResultExt;

use crate::error::SageError;
use crate::terminal::{print_info, print_warning};
use crate::utils::{get_extension_from_filename, get_files_list};

pub const TERRAFORM_EXTENSIONS: &'static [&str; 2] = &["tf", "tfvars"];

pub struct TerraformClient;

impl TerraformClient {
    // Initialize a new instance of Terraform client.
    pub fn new() -> TerraformClient {
        TerraformClient {}
    }

    // Extracts terraform arguments passed from the terminal.
    fn extract_arguments(&self, extra: &Vec<String>) -> Vec<String> {
        match extra.len() {
            count if count > 1 => extra[1..].to_vec(),
            _ => vec![],
        }
    }

    // Returns list of Terraform variable modules found in the specified directory.
    fn get_variable_modules(&self, directory: &String, args: &Vec<String>) -> Vec<String> {
        get_files_list(directory)
            .unwrap_or(vec![])
            .into_iter()
            .filter(|f| match f.metadata() {
                Ok(metadata) => metadata.is_file(),
                Err(_) => false,
            })
            .filter(|f| {
                let file_name = f.file_name().to_string_lossy().into_owned();
                !file_name.starts_with("out")
            })
            .filter(|f| {
                let path = f.path().to_string_lossy().into_owned();
                let file_extension = get_extension_from_filename(&path);
                TERRAFORM_EXTENSIONS
                    .iter()
                    .find(|&&ext| ext == file_extension)
                    .is_some()
            })
            .map(|f| {
                let path = f.path().to_string_lossy().into_owned();
                String::from(&format!("-var-file={}", path))
            })
            .filter(|arg| !args.clone().contains(arg))
            .collect()
    }

    // Prepares list of arguments, required for Terraform's init command.
    pub fn get_init_args(&self, directory: &String, extra: &Vec<String>) -> Vec<String> {
        let mut terraform_args = self.extract_arguments(extra);
        terraform_args.push(directory.to_string());
        terraform_args
    }

    // Prepares list of arguments, required for Terraform's plan command.
    pub fn get_plan_args(
        &self,
        config_directory: &String,
        directory: &String,
        extra: &Vec<String>,
    ) -> Vec<String> {
        let mut terraform_args = self.extract_arguments(extra);
        let mut variable_modules = self.get_variable_modules(config_directory, &terraform_args);
        terraform_args.append(&mut variable_modules);
        terraform_args.push(directory.to_string());
        terraform_args
    }

    // Invokes Terraform's command with the given `command` name and `args` arguments.
    // The output of this command is printing in user's terminal. In the case of any errors
    // also prints captured errors.
    pub fn call_without_input(&self, command: &str, args: &Vec<String>) -> Result<(), SageError> {
        print_info(&format!(
            "Executing command: `terraform {} {}`",
            command, args.join(" ")
        ));
        let process = Command::new("terraform")
            .arg(command)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(command)?;

        let copy_error = String::from("copying command output into buffer");
        let mut stderr_output = String::new();
        process
            .stderr
            .unwrap()
            .read_to_string(&mut stderr_output)
            .context(&copy_error)?;

        let mut stdin_output = String::new();
        process
            .stdout
            .unwrap()
            .read_to_string(&mut stdin_output)
            .context(&copy_error)?;

        print_info(&format!("Terraform output: \n{}", stdin_output));
        if !stderr_output.is_empty() {
            println!("{}", stderr_output);
        }
        Ok(())
    }

    // Deletes the file with the given path specified in `filepath` parameter.
    pub fn delete_main_tf(&self, filepath: &String) -> Result<(), SageError> {
        let delete_message = format!("Deleting {} file after execution...", filepath);
        print_warning(&delete_message);
        fs::remove_file(filepath).context(filepath)?;
        Ok(())
    }
}

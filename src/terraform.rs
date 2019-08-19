use std::fs;
use std::io::Read;
use std::process::{Command, Stdio};

use quick_error::ResultExt;

use crate::error::SageError;
use crate::terminal::{print_info, print_warning};

// Deletes the file with the given path specified in `filepath` parameter.
pub fn delete_terraform_file(filepath: &String) -> Result<(), SageError> {
    let delete_message = format!("Deleting {} file after execution...", filepath);
    print_warning(&delete_message);
    fs::remove_file(filepath).context(filepath)?;
    Ok(())
}

// Extracts terraform arguments passed from the terminal.
pub fn extract_terraform_arguments(extra: &Vec<String>) -> Vec<String> {
    match extra.len() {
        count if count > 1 => extra[1..].to_vec(),
        _ => vec![],
    }
}

// Prepare list of arguments, required for Terraform's init command.
pub fn get_terraform_init_args(directory: &String, extra: &Vec<String>) -> Vec<String> {
    let mut terraform_args = extract_terraform_arguments(extra);
    terraform_args.push(directory.to_string());
    terraform_args
}

// Invoke Terraform's command with the given `command` name and `args` arguments.
// The output of this command is printing in user's terminal. In the case of any errors
// also prints captured errors.
pub fn terraform_call_without_input(command: &str, args: &Vec<String>) -> Result<(), SageError> {
    print_info(&format!(
        "Executing command: `terraform init {}`",
        args.join(" ")
    ));
    let process = Command::new("terraform")
        .arg(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("init")?;

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

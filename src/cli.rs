use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "terraform-sage")]
pub enum Command {
    /// Initialize a Terraform working configuration
    #[structopt(
        name = "init",
        raw(setting = "structopt::clap::AppSettings::TrailingVarArg")
    )]
    Init {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "target",
            help = "Path to the main Terraform module (*.tf)"
        )]
        target: Option<String>,

        #[structopt(
            long = "template",
            default_value = "main.tpl",
            help = "File name of the used template module (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            help = "File name of the generated Terraform module (*.tf)"
        )]
        out: Option<String>,

        #[structopt(
            long = "--cleanup",
            help = "Delete main.tf module after initialization."
        )]
        cleanup: bool,

        #[structopt(hidden = true, help = "Extra options for Terraform init command")]
        extra: Vec<String>,
    },
    /// Generate an execution plan for Terraform
    #[structopt(
        name = "plan",
        raw(setting = "structopt::clap::AppSettings::TrailingVarArg")
    )]
    Plan {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "target",
            help = "Path to the main Terraform module (*.tf)"
        )]
        target: Option<String>,

        #[structopt(
            long = "template",
            default_value = "main.tpl",
            help = "File name of the used template module (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            help = "File name of the generated Terraform module (*.tf)"
        )]
        out: Option<String>,

        #[structopt(
            long = "--cleanup",
            help = "Delete main.tf module after initialization."
        )]
        cleanup: bool,

        #[structopt(hidden = true, help = "Extra options for Terraform plan command")]
        extra: Vec<String>,
    },
    /// Create or update infrastructure in according to Terraform configuration
    #[structopt(
        name = "apply",
        raw(setting = "structopt::clap::AppSettings::TrailingVarArg")
    )]
    Apply {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "target",
            help = "Path to the main Terraform module (*.tf)"
        )]
        target: Option<String>,

        #[structopt(
            long = "template",
            default_value = "main.tpl",
            help = "File name of the used template module (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            help = "File name of the generated Terraform module (*.tf)"
        )]
        out: Option<String>,

        #[structopt(
            long = "--cleanup",
            help = "Delete main.tf module after initialization."
        )]
        cleanup: bool,

        #[structopt(hidden = true, help = "Extra options for Terraform apply command")]
        extra: Vec<String>,
    },
    /// Destroy infrastructure managed by Terraform
    #[structopt(
        name = "destroy",
        raw(setting = "structopt::clap::AppSettings::TrailingVarArg")
    )]
    Destroy {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "target",
            help = "Path to the main Terraform module (*.tf)"
        )]
        target: Option<String>,

        #[structopt(
            long = "template",
            default_value = "main.tpl",
            help = "File name of the used template module (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            help = "File name of the generated Terraform module (*.tf)"
        )]
        out: Option<String>,

        #[structopt(
            long = "--cleanup",
            help = "Delete main.tf module after initialization."
        )]
        cleanup: bool,

        #[structopt(hidden = true, help = "Extra options for Terraform destroy command")]
        extra: Vec<String>,
    },
    /// Reads output variables from a Terraform state file and prints them.
    Output {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "target",
            help = "Path to the main Terraform module (*.tf)"
        )]
        target: Option<String>,

        #[structopt(
            long = "template",
            default_value = "main.tpl",
            help = "File name of the used template module (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            help = "File name of the generated Terraform module (*.tf)"
        )]
        out: Option<String>,

        #[structopt(
            long = "--cleanup",
            help = "Delete main.tf module after initialization."
        )]
        cleanup: bool,

        #[structopt(hidden = true, help = "Extra options for Terraform destroy command")]
        extra: Vec<String>,
    },
    #[structopt(name = "list")]
    /// Show available configurations
    List {
        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,
    },
    #[structopt(name = "generate")]
    /// Generate main.tf from the template file
    Generate {
        #[structopt(required = true, help = "Configuration name")]
        config: String,

        #[structopt(
            short = "d",
            long = "dir",
            default_value = ".",
            help = "Path to directory with Terraform files"
        )]
        directory: String,

        #[structopt(
            short = "t",
            long = "template",
            default_value = "main.tpl",
            help = "Path to the template file (*.tpl)"
        )]
        template: String,

        #[structopt(
            short = "o",
            long = "out",
            default_value = "main.tf",
            help = "Path to the generated file (*.tf)"
        )]
        out: String,
    },
}

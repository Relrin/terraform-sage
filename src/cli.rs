use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "terraform-sage")]
pub enum Command {
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
            long = "target",
            default_value = "main.tpl",
            help = "Path to the used template file (*.tpl)"
        )]
        target: String,
        #[structopt(
            short = "o",
            long = "out",
            default_value = "main.tf",
            help = "Path to the generated file"
        )]
        out: String,
    },
}

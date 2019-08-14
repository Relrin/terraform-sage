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
            help="Path to directory with Terraform files"
        )]
        directory: String
    }
}

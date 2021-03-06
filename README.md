# terraform-sage

Cross-platform tool for easier Terraform deployments

 - [Quick start](#quick-start)
 - [FAQ](#faq)
 - [Advanced usage](#advanced-usage)
 - [Project structure](#project-structure)
 - [Development](#development)
 - [License](#license)

## Features

- Template-based approach for work in multiple environments
- Semi-automated deploys via command-line interface (as Terraform does)

## Requirements

Terraform >= 0.11 (older not tested)

## Quick start

1. Install [Terraform](https://learn.hashicorp.com/terraform/getting-started/install.html).

2. Download executable/binary file in according to the used operation system from the [releases page](https://github.com/Relrin/terraform-sage/releases).

3. Link executable/binary file to operation system, so you could invoke `terraform-sage` everywhere:

    - Linux / Mac OS
  
        Move the binary file to the `/usr/local/bin` directory and restart the terminal
        ```
        mv ~/Downloads/terraform-sage /usr/local/bin
        ```
    
    - Windows
    
        1. Right click on the Windows Logo and select the `System` menu item.
        2. Click on the `Advanced System Settings` button.
        3. Click on the `Environment Variables` button.
        4. Select your `PATH` variable and click in the `Edit` button.
        5. Click on the `New` button.
        6. Add the file path to the directory with the `terraform-sage` executable.
        7. Click on the `OK` button a couple of times for applying changes.

4. Go to your directory with code and create a directory for terraform files. For example, let's name it `terraform`.

5. Inside of the `terraform` directory, create the `configs` directory. This directory will store all required Terraform modules related for each environment. So you will get something like this:
    ```
    <sources>
    ├ docker
    ├ microservice
    └ terraform
      └ configs
        ├ dev
        │  ├ ... 
        │  └ variables.tf
        ├ staging
        │  ├ ...
        │  └ variables.tf
        └ production
           ├ ...
           └ variables.tf
    ```
    The `configs` directory is required and used for correct and smooth work of this wrapper. The `terraform-sage` will track environments defined in `configs` directory and will make an according action depends on the invoked Terraform command.
    
    P.S. Also see the [project structure](#project-structure) section for more information about recommended project structure.

6. Create the base template (I recommend to name it as the `main.tpl`), that stores main definition of your resources and a backend storage for Terraform state. For making the writing base template easier, I recommend you to start with from `main.tf` module where you will describe everything what you need and then rename it to `main.tpl`. After when all resources / modules are described you will need to specify one of available [backend storages](https://www.terraform.io/docs/backends/types/index.html) for Terraform states, and then append `{{CONFIG_NAME}}` string to the key (which is basically is the file name for Terraform state), like this:
    ```
    terraform {
      backend "s3" {
        bucket = "state-bucket"
        key    = "terraform/state-{{CONFIG_NAME}}.tfstate"
        region = "us-east-1"
      }
    }
    ```
    The `{{CONFIG_NAME}}` string is using as the template parameter that will be replaced on the used environment name during the `terraform-sage` command call. This feature will help us in handling different environments without duplicating source code of the `main.tf` file. 

7. Then, instead of direct calls to Terraform CLI, you can use the following commands:
    ```
    terraform-sage init
    terraform-sage plan
    terraform-sage apply
    terraform-sage destroy
    terraform-sage output
    ```
    Also CLI provides two additional commands for generating Terraform's `main.tf` modules and retrieving the list of available environments.
    ```
    terraform-sage generate
    terraform-sage list
    ```
    For more information about acceptable arguments and options for each command, call any desired command with the `--help` option.

## F.A.Q.
**Q**: Why this tool / wrapper was written?  
**A**: Being a quite lazy developer, I tired of writing long Terraform command calls with specifying used `*.tf` / `*.tfvars` modules when you need to create or update something on any existing environment. The mess with modules increases when developers are using different operation systems and "additional" scripts for simplifying their Terraform workflows.

**Q**: How can I pass extra arguments for Terraform command (e.g. `terraform-sage apply`)?  
**A**: For this case you will need to append `.` in the end, and then specify a list of arguments as you were working with actual Terraform CLI, like this:
```
terraform apply dev --dir=examples/approach_two . -var-file=my-variables.tf
```

## Advanced usage
Terraform-sage tool also provides a way to define additional context for each used environment. For this you will need to create a new file with the `context.toml` name in the root configs folder. As the result we will have the following project structure:
```
terraform
├ configs
│  ├ dev
│  │  ├ secrets.tf
│  │  └ variables.tf
│  ├ staging
│  │  ├ secrets.tf
│  │  └ variables.tf
│  └ production
│  │  ├ secrets.tf
│  │  └ variables.tf
│  └  context.toml
└ main.tpl
```

The `context.toml` file contains placeholders for the template engine which is using during the generation of the `main.tf` file. Each section has the same name as the folder with variables for your environment. Under each section you need to specify key-pair values that supposed to be used in the appropriate context.

An example the definitions:
```
[dev]
bucket_name = "dev"
profile = "aws-dev-account"

[product]
bucket_name = "prod"
profile = "aws-prod-account"
```
After it, you will need to declare placeholders in the appropriate places in the `main.tpl`, like this:
```
provider "aws" {
  profile = "{{profile}}"
  region  = "us-east-1"
}

# ...

resource "aws_s3_bucket" "bucket" {
  bucket = "terraform-sage-bucket"
  acl    = "private"

  tags = {
    "Name": "{{bucket_name}}"
    "EnvironmentType" = "${var.environment}"
  }
}
```
And then you're ready to go! Just call the `terraform-sage apply dev` command (or any other suitable to you) without any further changes.

## Project structure

The `terraform-sage` application relies on the certain project structure for a correct work. Therefore, I recommend to developers two ways of organizing their own projects:
- Approach #1:

  ```
  terraform
  ├ configs
  │  ├ dev
  │  │  ├ secrets.tf
  │  │  └ variables.tf
  │  ├ staging
  │  │  ├ secrets.tf
  │  │  └ variables.tf
  │  └ production
  │     ├ secrets.tf
  │     └ variables.tf
  ├ resources
  │  ├ rds
  │  │  ├ main.tf
  │  │  ├ output.tf
  │  │  └ variables.tf
  │  └ sqs
  │     ├ main.tf
  │     ├ output.tf
  │     └ variables.tf
  └ main.tpl
  ```

  Pros:

  - Dependant resources are splitted into [Terraform modules](https://www.terraform.io/docs/configuration/modules.html), therefore it is easier to re-use in other projects.
  - Easier to manage projects which have a lot of dependencies.
  - `main.tpl` file is relatively small and contains only a minimum amount of code

  Cons:

  - Requires more boilerplate code when providing extra arguments from the main executable Terraform configuration to dependant resources.

- Approach #2:

  ```
  terraform
  ├ configs
  │  ├ dev
  │  │  ├ secrets.tf
  │  │  └ variables.tf
  │  ├ staging
  │  │  ├ secrets.tf
  │  │  └ variables.tf
  │  └ production
  │     ├ secrets.tf
  │     └ variables.tf
  └ main.tpl
  ```

  Pros:

  - A good choice for small projects with a couple of resources
  - Easy to pass arguments to dependant resources

  Cons:

  - During the evolution of the project, `main.tpl` file can contain a lot of dependant resources

## Development

To start developing you will need:

- [Docker](https://docs.docker.com/install/)
- [Docker-compose](https://docs.docker.com/compose/install/)

Before attaching to the node, you will need to build up the local dev image and start it in the detached mode. Run the following command from the project root folder:

```
docker-compose -f docker-compose.dev.yml up -d
```

Then connect to the `app` node with bash via `exec` command:

```
docker-compose -f docker-compose.dev.yml exec app bash
```

And now, you're ready to go! Use the `cargo` tool command inside of the container as you would like.

## License

The terraform-sage project is published under BSD license. For more details read the [LICENSE](https://github.com/Relrin/terraform-sage/blob/master/LICENSE) file.

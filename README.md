# terraform-sage

Cross-platform tool for easier Terraform deployments

## Features

- Cross-platform wrapper for Terraform written in Rust
- Template-based approach for working in multiple environments
- Semi-automated deploys via command-line interface

## Requirements

Terraform >= 0.11 (older not tested)

## Project structure

The `terraform-sage` application recommends to developers two ways to organize their own projects:

- Approach #1:

  ```
  terraform
  ├ configs
  │  ├ dev
  │  │  ├ secrets.tfvars
  │  │  └ variables.tfvars
  │  ├ staging
  │  │  ├ secrets.tfvars
  │  │  └ variables.tfvars
  │  └ production
  │     ├ secrets.tfvars
  │     └ variables.tfvars
  ├ resources
  │  ├ rds
  │  │  ├ main.tf
  │  │  ├ output.tf
  │  │  └ variables.tfvars
  │  └ sqs
  │     ├ main.tf
  │     ├ output.tf
  │     └ variables.tfvars
  └ main.tpl
  ```

  Pros:

  - Dependant resources are split into [Terraform modules](https://www.terraform.io/docs/configuration/modules.html), therefore it is easier to re-use in other projects.
  - Easier to manage projects which have a lot of dependencies.
  - `main.tf` file is relatively small and contains only a minimum amount of code

  Cons:

  - Requires more boilerplate code when providing extra arguments from the main executable Terraform configuration to dependant resources.

- Approach #2:

  ```
  terraform
  ├ configs
  │  ├ dev
  │  │  ├ secrets.tfvars
  │  │  └ variables.tfvars
  │  ├ staging
  │  │  ├ secrets.tfvars
  │  │  └ variables.tfvars
  │  └ production
  │     ├ secrets.tfvars
  │     └ variables.tfvars
  └ main.tpl
  ```

  Pros:

  - Good choice for small projects with a couple of resources
  - Easy to pass arguments to dependant resources

  Cons:

  - During the evolution of the project, `main.tf` file can contain a lot of dependant resources

## Development

To start developing you will need:

- [Docker](https://docs.docker.com/install/)
- [Docker-compose](https://docs.docker.com/compose/install/)

Before attaching to the node, you will need to build the local dev image and start it in detached mode. Run the following command from the project root folder:

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

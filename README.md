# terraform-sage
Cross-platform tool for easier Terraform deployments

Futures
--------
- Semi-automated deploys via command-line interface

Requirements
------------
Terraform >= 0.11 (older not tested)

Project structure
-----------------
The `terraform-sage` application provides to developers two ways to organize their own project:
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
    └ main.tf
    ```
    Pros:  
        - Dependant resources are splitted onto [Terraform modules](https://www.terraform.io/docs/configuration/modules.html), therefore it is easier to re-use in other projects.    
        - Easier to manage projects which has a lot of dependencies.  
        - `main.tf` file is relatively small and contains only the minimum amount of code  
 
    Cons:  
        - Requires more boilerplate code when necessary provide extra arguments from the main executable Terraform configuration to dependant resources.

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
    └ main.tf
    ```
    Pros:  
        - Good choice for small projects with a couple of resources  
        - Easy to pass argument to dependant resources
        
    Cons:  
        - During evolving the project `main.tf` file can contain a lot of dependant resources

License
-------
The terraform-sage project is published under BSD license. For more details read the [LICENSE](https://github.com/Relrin/terraform-sage/blob/master/LICENSE) file.

provider "aws" {
  profile = "default"
  region  = "us-east-1"
}

terraform {
  backend "local" {
    path = "./terraform-{{CONFIG_NAME}}.tfstate"
  }
}

resource "aws_instance" "micro_ec2_instance" {
  ami           = "ami-2757f631"
  instance_type = "t2.micro"

  tags = {
    "Name" = "test-instance"
    "EnvironmentType" = "${var.environment}"
  }
}

provider "aws" {
  profile = "default"
  region  = "us-east-1"
}

terraform {
  backend "local" {
    path = "./terraform-{{CONFIG_NAME}}.tfstate"
  }
}

resource "aws_s3_bucket" "bucket" {
  bucket = "terraform-sage-bucket"
  acl    = "private"

  tags = {
    "EnvironmentType" = "${var.environment}"
  }
}

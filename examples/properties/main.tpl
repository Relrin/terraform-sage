provider "aws" {
  profile = {{profile}}
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
    "Name": {{aws_bucket_name}}
    "EnvironmentType" = "${var.environment}"
  }
}

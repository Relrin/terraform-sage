provider "aws" {
  profile    = "default"
  region     = "us-east-1"
}


resource "aws_instance" "micro_ec2_instance" {
  ami           = "ami-2757f631"
  instance_type = "t2.micro"

  tags = {
    "EnvironmentType" = "${var.environment}"
  }
}

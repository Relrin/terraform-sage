FROM rust:1.36-stretch

# Install Terraform CLI
RUN apt-get update && apt-get install unzip
RUN wget https://releases.hashicorp.com/terraform/0.11.14/terraform_0.11.14_linux_amd64.zip
RUN unzip terraform_0.11.14_linux_amd64.zip
RUN install terraform /usr/local/bin/

COPY ./ /code
WORKDIR /code

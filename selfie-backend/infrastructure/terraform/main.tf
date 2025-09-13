terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
      version = "~> 5.0"
    }}
  }}
}}

provider "aws" {{
  region = var.region
}}

resource "aws_vpc" "selfie" {{
  cidr_block = "10.0.0.0/16"
}}

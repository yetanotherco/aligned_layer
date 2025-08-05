#!/bin/bash
python3 /etc/localstack/init/ready.d/init-s3.py \
  --endpoint_url http://localhost:4566 \
  --access_key test \
  --secret_key test \
  --bucket_name aligned.storage \
  --region us-east-2

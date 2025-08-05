#!/bin/bash
python3 /etc/localstack/init/ready.d/init-s3.py \
  --endpoint_url http://localhost:4566 \
  --access_key test2 \
  --secret_key test2 \
  --bucket_name aligned.storage \
  --region us-west-1

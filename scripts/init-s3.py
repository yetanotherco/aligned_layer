import boto3
import argparse

def main():
    parser = argparse.ArgumentParser(description='Initialize S3 bucket in LocalStack')
    parser.add_argument('--endpoint_url', default='http://localhost:4566', help='S3 endpoint URL')
    parser.add_argument('--access_key', default='test', help='AWS access key ID')
    parser.add_argument('--secret_key', default='test', help='AWS secret access key')
    parser.add_argument('--bucket_name', default='aligned.storage', help='S3 bucket name')
    parser.add_argument('--region', default='us-east-2', help='AWS region')
    
    args = parser.parse_args()

    s3_client = boto3.client(
        "s3",
        endpoint_url=args.endpoint_url,
        aws_access_key_id=args.access_key,
        aws_secret_access_key=args.secret_key,
        region_name=args.region
    )

    try:
        s3_client.create_bucket(
            Bucket=args.bucket_name,
            CreateBucketConfiguration={'LocationConstraint': args.region}
        )
        print(f"Successfully created bucket: {args.bucket_name}")
    except Exception as e:
        print(f"Error creating bucket {args.bucket_name}: {e}")

if __name__ == "__main__":
    main()

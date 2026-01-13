
# Init

```shell
source .env && terraform init -migrate-state \
    -backend-config="bucket=${TFSTATE_BUCKET}" \
    -backend-config="key=${TFSTATE_KEY}" \
    -backend-config="region=${TFSTATE_REGION}"
```

# Plan

```shell
source .env && terraform plan
```

# Apply

```shell
source .env && terraform apply
```


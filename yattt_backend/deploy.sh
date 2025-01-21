#!/bin/bash

set -e -x -o pipefail

# Set namespace and filenames
NAMESPACE="yattt-backend"
ENV_FILE=".env"
SECRET_NAME="surrealdb-credentials"
DEPLOYMENT_FILE="yattt_backend_deployment.yml"  # Your single deployment file containing both services

# Create the namespace
kubectl apply -f - <<EOF
apiVersion: v1
kind: Namespace
metadata:
  name: $NAMESPACE
EOF

# Check if .env file exists
if [ ! -f "$ENV_FILE" ]; then
  echo "Error: $ENV_FILE file not found!"
  exit 1
fi

# Create the Kubernetes secret from the .env file
echo "Creating Kubernetes secret from $ENV_FILE..."
kubectl create secret generic "$SECRET_NAME" \
  --from-env-file="$ENV_FILE" \
  --namespace="$NAMESPACE" \
  --dry-run=client -o yaml | kubectl apply -f -

if [ $? -ne 0 ]; then
  echo "Error: Failed to create the secret."
  exit 1
fi

# Apply the deployment file (containing both SurrealDB and Rust service)
echo "Applying deployment from $DEPLOYMENT_FILE..."
kubectl apply -f "$DEPLOYMENT_FILE" --namespace="$NAMESPACE"

if [ $? -ne 0 ]; then
  echo "Error: Failed to apply the deployment."
  exit 1
fi

# Wait for SurrealDB pod to be running
echo "Waiting for SurrealDB pod to be running..."
kubectl rollout status deployment/surrealdb --namespace="$NAMESPACE"

if [ $? -ne 0 ]; then
  echo "Error: SurrealDB deployment failed to start."
  exit 1
fi

# Wait for Rust service pod to be running
echo "Waiting for Rust service pod to be running..."
kubectl rollout status deployment/rust-service --namespace="$NAMESPACE"

if [ $? -ne 0 ]; then
  echo "Error: Rust service deployment failed to start."
  exit 1
fi

# Verify the deployment
echo "Verifying the deployments..."
kubectl get pods --namespace="$NAMESPACE"
kubectl get services --namespace="$NAMESPACE"

echo "Deployment complete! SurrealDB and Rust service should be running."
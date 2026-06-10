#!/bin/bash
set -e

echo "Creating and funding testnet deployer account..."
stellar keys generate alice --network testnet --fund
echo "Deployer address: $(stellar keys address alice)"
echo "Done. Wait 10 seconds for funding to confirm before running deploy.sh"

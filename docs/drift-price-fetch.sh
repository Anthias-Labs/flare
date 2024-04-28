#!/bin/bash

# Read the JSON file, get the address of the provided ticker
address=$(cat drift-perp-addresses.json | jq .$1)

# Read account with the corresponding address from Drift
# Send that result to `jq`, and grab the latest price
price=$(flare read-account \
        --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
        --account ${address:1:44} \
        --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json \
        | jq .amm.historicalOracleData.lastOraclePrice)


# Use `bc` to divide the price, with a 4-digit precision
echo "scale=4; $price / 1000000" | bc -l
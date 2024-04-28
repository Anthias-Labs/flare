# Reading Drift prices

In this document, we will show how you can use Flare to read real-time pricing for coins from [Drift](https://www.drift.trade/#). We will also show how you can integrate Flare into useful scripts, without needing to learn or rely on external libraries.

For documentation of all of Flare's commands, see [here](./README.md).

**Note**: This tutorial assumes basic Flare knowledge, as well as some basic programming and Bash experience. For beginners, we recommend starting with the [on chain voting example](./on-chain-voting.md).

We will use the mainnet deployment of Drift at `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH`.

## Getting the IDL

First, we will want to get a copy of the IDL of the program. Alternatively, if we ommit it from the commands, Flare will automatically fetch it on each call, but this will be slower. Fortunately, we can get the IDL with Flare by running:

```bash
flare fetch-idl --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH
```

This will output the IDL and save it to a file called `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json`,

## Reading the Perp Market account

Using an account indexer, we can get the addresses for the `PerpMarket` accounts for all the tokens. For convenience, here are the addresses:
```
SOL - 8UJgxaiQx5nTrdDgph5FiahMmzduuLTLf5WmsPegYA6W
ETH - 25Eax9W8SA3wpCQFhJEGyHhQ2NDHEshZEDzyMNtthR8D
SEI - EBsU7BPiCDw7Q7GqQBzNcFGdFDU9bEmE4TjuW76fA62r
JUP - 8DdB5hHSZtPT3oqbsiHUytCrrodApNC31k3MuZhxJH61
APT - 7QAtMC3AaAc91W4XuwYXM1Mtffq9h9Z8dTxcJrKRHu1z
AVAX - CGVDM9FjRQR7e1oV8cAitnYJNZdKo7szzSSLcxhLgJEx
MATIC - 6oopaUD3RK7mHBf2vPWT3aUodysg3VWcqksuAZo4xWrt
RNDR - 6KPv8DdWauTCV2zMqqiUbP1MjqSCDnA453VodUtZCFZR
OP - Aw9bzBKbryKnoLtYRLuhbhEYZHcoZyxZ5XszdepwHRKJ
LINK - 3a7HAEqxzwvJEAViYKhDtHk85mrFf1dU2HCsffgXxUj8
XRP - 2fqYPht3DVWKHuEzPJy4eaCzd5onZhw7fwSxpGohexNm
WIF - 8BbCGbxsQk1HYohgdn1TMUNs6RYcX4Hae3k8mt4rvnzf
BTC - 2UZMvVTBQR9yWxrEdzEQzXWE61bUjqQ5VpJAGqVb3B19
JTO - FH6CkSYthofVKdfuNagWn48fou1Dq5REkxhtZsk22Gpi
PYTH - 75Mk3ySkJG5rCAsiQd4KZfFws35dSj2JVa6jxrqyTM52
HNT - 7jyQomwaLZYpwrcZWAa7yoDeLPTsXsCDEzhvtxeee5hY
1MPEPE - GsMte91Y1eY9XYtY1nt1Ax77V5hzsj3rr1a7a29mxHZw
DOGE - 48R9ic9xgigVRqNPbABN8gTGoRV9wn6UUmcKYz3csbhR
SUI - 91NsaUmTNNdLGbYtwmoiYSn9SgWHCsZiChfMYMYZ2nQx
INJ - 2uBzNiiGJvJhK2iuZZKJcCZH9ih1kFroq3ZPqo9UYDUU
TIA - H9AGF2BJe35YYgwjF8oZZogQxwBmBEy3soZWZpkVZq9e
ARB - 53xRgYi7591y8TKSqRbC2AMzXJF7ZLLTms6t2XKuigUp
RLB - CZtHZuoLWdPYZNGan5PW9P2VEnzsywgkVy1Vfe6nMN5o
```

We also provide a [drift-perp-addresses.json](./drift-perp-addresses.json) file for convenience.

Let's try to read the SOL account:
```bash
flare read-account \
    --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
    --account 8UJgxaiQx5nTrdDgph5FiahMmzduuLTLf5WmsPegYA6W \
    --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json
```

We get the following output:
```json
{"pubkey":"8UJgxaiQx5nTrdDgph5FiahMmzduuLTLf5WmsPegYA6W","amm":{"oracle":"J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix","historicalOracleData":{"lastOraclePrice":142135150,"lastOracleConf":0,"lastOracleDelay":4,"lastOraclePriceTwap":141583246,"lastOraclePriceTwap5min":142060335,"lastOraclePriceTwapTs":1714268048},"baseAssetAmountPerLp":"22711020857141","quoteAssetAmountPerLp":"-2871554168152","feePool":{"scaledBalance":"495542913354924","marketIndex":0,"padding":[0, 0, 0, 0, 0, 0]},"baseAssetReserve":"974789616","quoteAssetReserve":"1867974","concentrationCoef":"1020710","minBaseAssetReserve":"964808433","maxBaseAssetReserve":"1005184608","sqrtK":"42671798","pegMultiplier":"74104404084","terminalQuoteAssetReserve":"1849006","baseAssetAmountLong":"1056790000000","baseAssetAmountShort":"-1056780000000","baseAssetAmountWithAmm":"10000000","baseAssetAmountWithUnsettledLp":"0","maxOpenInterest":"5000000000000000","quoteAssetAmount":"20200428165","quoteEntryAmountLong":"-140821683709","quoteEntryAmountShort":"155215360572","quoteBreakEvenAmountLong":"-140743347371","quoteBreakEvenAmountShort":"156429959823","userLpShares":"0","lastFundingRate":18671083,"lastFundingRateLong":18671083,"lastFundingRateShort":18671083,"last24hAvgFundingRate":-55921298,"totalFee":"423047824457","totalMmFee":"383995504633","totalExchangeFee":"40913113981","totalFeeMinusDistributions":"196738610244","totalFeeWithdrawn":"161987080975","totalLiquidationFee":"397542262743","cumulativeFundingRateLong":"31167501555","cumulativeFundingRateShort":"-1812791805","totalSocialLoss":"735346959028","askBaseAssetReserve":"971125855","askQuoteAssetReserve":"1875022","bidBaseAssetReserve":"975293300","bidQuoteAssetReserve":"1867010","lastOracleNormalisedPrice":142084470,"lastOracleReservePriceSpreadPct":-915,"lastBidPriceTwap":139608816,"lastAskPriceTwap":139608816,"lastMarkPriceTwap":139608816,"lastMarkPriceTwap5min":138628185,"lastUpdateSlot":295143489,"lastOracleConfPct":356,"netRevenueSinceLastFunding":335859,"lastFundingRateTs":1714176000,"fundingPeriod":3600,"orderStepSize":10000000,"orderTickSize":100,"minOrderSize":10000000,"maxPositionSize":0,"volume24h":9113146810,"longIntensityVolume":0,"shortIntensityVolume":3922070478,"lastTradeTs":1714179355,"markStd":1216943,"oracleStd":501224,"lastMarkPriceTwapTs":1714179355,"baseSpread":1000,"maxSpread":5000,"longSpread":7537,"shortSpread":1032,"longIntensityCount":0,"shortIntensityCount":3,"maxFillReserveFraction":100,"maxSlippageRatio":50,"curveUpdateIntensity":200,"ammJitIntensity":200,"oracleSource":"Pyth","lastOracleValid":true,"targetBaseAssetAmountPerLp":0,"perLpBase":3,"padding1":0,"padding2":0,"totalFeeEarnedPerLp":840554257,"netUnsettledFundingPnl":-101586330,"quoteAssetAmountWithUnsettledLp":-33553980963060,"referencePriceOffset":0,"padding":[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},"pnlPool":{"scaledBalance":"753094913295910","marketIndex":0,"padding":[0, 0, 0, 0, 0, 0]},"name":[83, 79, 76, 45, 80, 69, 82, 80, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32],"insuranceClaim":{"revenueWithdrawSinceLastSettle":0,"maxRevenueWithdrawPerPeriod":1000,"quoteMaxInsurance":1000,"quoteSettledInsurance":1000,"lastRevenueWithdrawTs":1710882320},"unrealizedPnlMaxImbalance":10000,"expiryTs":5129292360,"expiryPrice":119160063,"nextFillRecordId":1326021,"nextFundingRateRecordId":9172,"nextCurveRecordId":1418,"imfFactor":1,"unrealizedPnlImfFactor":1,"liquidatorFee":10000,"ifLiquidationFee":10000,"marginRatioInitial":500,"marginRatioMaintenance":400,"unrealizedPnlInitialAssetWeight":0,"unrealizedPnlMaintenanceAssetWeight":1,"numberOfUsersWithBase":23,"numberOfUsers":30,"marketIndex":0,"status":"Active","contractType":"Perpetual","contractTier":"A","pausedOperations":0,"quoteSpotMarketIndex":0,"feeAdjustment":-75,"padding":[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}
```

There is a lot of information there! Importantly, we can see that the price is present at `amm > historicalOracleData > lastOraclePrice`.

We can use [jq](https://jqlang.github.io/jq/), a CLI for JSON parsing, to get this field:

```bash
flare read-account \
    --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
    --account 8UJgxaiQx5nTrdDgph5FiahMmzduuLTLf5WmsPegYA6W \
    --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json | jq .amm.historicalOracleData.lastOraclePrice
```

And we get something like:
```
142314967
```

Notice that the price is expressed as an integer, and we need to divide by $1000000$ to get the price in USD.

We will now see how we can integrate this into a script to fetch the prices. We provide two examples, one in Bash and one in Python.

## Writing a Bash Script

This script will be a simple CLI tool to fetch the price of a given coin.

Firstly, we will grab the address of the right `PerpMarket` account from the ticker:

```bash
#!/bin/bash

# Read the JSON file, get the address of the provided ticker
address=$(cat drift-perp-addresses.json | jq .$1)
```

We can now use a command similar to the one before to read this account and grab the latest price:

```bash
# Read account with the corresponding address from Drift
# Send that result to `jq`, and grab the latest price
price=$(flare read-account \
        --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
        --account ${address:1:44} \
        --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json \
        | jq .amm.historicalOracleData.lastOraclePrice)
```

Finally, we divide this price by $1000000$ to get the price in USD:

```bash
# Use `bc` to divide the price, with a 4-digit precision
echo "scale=4; $price / 1000000" | bc -l
```

The final script looks like this:
```bash
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
```


You can find it [here](./drift-price-fetch.sh).

We can now save it to a file and use it as such:
```bash
./drift-price-fetch.sh SOL
```
And we get:
```
143.2017
```

## Writing a Python script

The Python verion will be a bit different, and output a table showing the current price for every token.

First, we will write a quick function to interface with Flare by calling shell commands:

```python
# Simple wrapper to Flare
def flare(command):
    out = subprocess.check_output(f"flare {command}", shell=True, text=True)
    return out.strip()
```

This allows us to run things like `flare("epoch")` and get the output.

:rotating_light: **WARNING**: This function is unsafe, and vulnerable to injections in user facing software. Do not use in production. As in this script we control the input, we will still use it.

Now, we can read the account addresses from the JSON file:
```python
# Load address JSON
addressFile = open("drift-perp-addresses.json")
addresses = json.loads(addressFile.read())
addressFile.close()
```

We are now ready to iterate over the addresses and fetch the price for each token, using the same Flare command as before:
```python
prices = {}

# Grab price for each token
for tok, addr in addresses.items():
    prices[tok] = flare(f"""read-account \
        --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
        --account {addr} \
        --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json \
        | jq .amm.historicalOracleData.lastOraclePrice""")
```

Finally, we can format the prices into a nice table for output:
```python
# Output table with all prices
print("+---------------------------------+")
print("|  Token         |  Price         |")
print("+---------------------------------+")

for tok, price in prices.items():
    print("|{:<16}|{:>16}|".format(tok, int(price) / 1_000_000))

print("+---------------------------------+")
```

Here is the final Python script:
```python
import json, subprocess

# WARNING: This is unsafe and vulnerable to injections. Do not use any user generated argument unless it was properly escaped
# Simple wrapper to Flare
def flare(command):
    out = subprocess.check_output(f"flare {command}", shell=True, text=True)
    return out.strip()

# Load address JSON
addressFile = open("drift-perp-addresses.json")
addresses = json.loads(addressFile.read())
addressFile.close()

prices = {}

# Grab price for each token
for tok, addr in addresses.items():
    prices[tok] = flare(f"""read-account \
        --program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH \
        --account {addr} \
        --idl dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH.json \
        | jq .amm.historicalOracleData.lastOraclePrice""")

# Output table with all prices
print("+---------------------------------+")
print("|  Token         |  Price         |")
print("+---------------------------------+")

for tok, price in prices.items():
    print("|{:<16}|{:>16}|".format(tok, int(price) / 1_000_000))

print("+---------------------------------+")
```

You can find it [here](./drift-all-prices.py).

And we get the following output:
```
+---------------------------------+
|  Token         |  Price         |
+---------------------------------+
|SEI             |        0.875904|
|JUP             |        0.200001|
|APT             |       16.461191|
|ETH             |        3066.005|
|AVAX            |          47.755|
|MATIC           |        0.681268|
|RNDR            |         11.5395|
|OP              |        3.165707|
|LINK            |       17.368749|
|XRP             |        0.491361|
|WIF             |        1.821506|
|BTC             |    61159.634999|
|JTO             |        4.250084|
|PYTH            |        0.577642|
|HNT             |        8.350999|
|SOL             |      143.256823|
|1MPEPE          |           5.035|
|DOGE            |        0.157534|
|SUI             |        1.900339|
|INJ             |       29.517975|
|TIA             |       13.732649|
|ARB             |        1.083207|
|RLB             |         0.12075|
+---------------------------------+
```
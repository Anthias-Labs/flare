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
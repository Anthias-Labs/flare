# Flare Usage

Flare provides a set of tools to interact with the Solana network, both for users and developers. For building and installing, please see [the project README](../README.md).

## Examples

For some usage examples, see [the example usage script](../example-usage.sh), or one of our project examples:
- [On chain voting](./on-chain-voting.md)
- [Self-custodial Facebook](./self-custodial-facebook.md)
- [Using Flare to fetch Drift prices - script integration](./drift-prices.md)

## General usage

Flare provides some common options for most commands:
- `--cluster [cluster]`: By default, Flare tries to read the system's Solana CLI config file and uses that cluster. If the Solana config is not found, `mainnet` is used by default. This option allows you to select a different cluster, either by name (`mainnet`, `devnet`, or `testnet`) or by an RPC URL.
- Commands that interact with the blockchain require a designated signer. There are two ways of specifying it:
    - `--mnemonic "[MNEMONIC]"`: Provide the mnemonic for the signer keypair.
    - `--keypair [KEYPAIR FILE]`: Provide the path to a JSON file with the private key, as generated by Flare or the Solana CLI.
- `--finalized` will tell Flare to wait for transactions to be finalized (as opposed to the default of confirmed) when sending a transaction to the cluster.

### Commands

#### Send
`flare send --to [TO] [AMOUNT]`: Requires mnemonic or keypair. Sends the specified SOL `AMOUNT`, in Lamports, to the specified `TO` address.

**Example**: `flare --cluster devnet send --keypair keypair.json --to 67pUbVFbg4Q94id7NgLFiC5GdtWKgmKYgNXp2CtwsWty 100`

#### Balance
`flare balance [PUBKEY]`: Outputs the SOL balance of the `PUBKEY` address, in Lamports.

**Example**: `flare --cluster devnet balance 67pUbVFbg4Q94id7NgLFiC5GdtWKgmKYgNXp2CtwsWty`

#### Epoch
`flare epoch`: Outputs the current epoch number of the cluster.


#### Block height
`flare block-height`: Outputs the current block height of the cluster.

#### Fetch IDL
`flare fetch-idl --program [PROGRAM]`: If published, fetches the IDL of the specified `PROGRAM` address, outputs it, and saves it to a JSON file.

**Example**:  `./flare --cluster devnet fetch-idl --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa`

#### Wallet Create
`flare wallet-create`: Generates a new random keypair, with a corresponding mnemonic. Also stores the keypair as a JSON file, in the same format as the Solana CLI.

#### Wallet Recover
`flare wallet-recover --mnemonic "[MNEMONIC]"`: Recovers the private key and address from a given `MNEMONIC`. Stores the keypair in a JSON file, in the same format as the Solana CLI.

**Example**: `flare wallet-recover --mnemonic "wreck open catch flame direct one depend steel spell chalk iron barely"`

#### Sign
`flare sign "[MESSAGE]"`: Requires mnemonic or keypair. Produces a signature from a given `MESSAGE` with the provided keypair.

**Example**: `flare sign --keypair keypair.json "Hello Solana!"`

#### Address derive
`flare address-derive --keypair [KEYPAIR FILE PATH]`: Given a `KEYPAIR FILE PATH`, in the same format as the Solana CLI, it outputs the address of the account.

**Example**: `flare address-derive --keypair keypair.json`

#### Generate PDA
`flare generate-pda --program [PROGRAM] [SEEDS]`: Given a `PROGRAM` address, it generates a Program Derived Account address from the given comma separated `SEEDS`. It infers the type by trying to parse it as an address, an integer, or a custom string, in that order. 

**Example**: `flare generate-pda --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve 9z7fAqY3Wj1ZQL2TBTUHqkFZ8CmEcNe4omwHmFwA29iL,foo,bar`

#### Call
Executes a method from an Anchor program. Requires mnemonic or keypair. There are two possible ways of using this command, given a certain `PROGRAM` address, `METHOD NAME`, and comma separated, flattened, list of `ARGS`:

- **Inline**: `flare call --program [PROGRAM] --accounts [ACCOUNTS] --signers [SIGNERS] --idl [IDL] [METHOD NAME] [ARGS]`

Where `ACCOUNTS` and `SIGNERS` are comma separated

**Example**:
```bash
./flare call --keypair keypair.json \
             --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve \
             --accounts 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA,9z7fAqY3Wj1ZQL2TBTUHqkFZ8CmEcNe4omwHmFwA29iL \
             --signers keypair.json \
             --idl ./example/onchain_voting.json \
              gibVote GM
```

- **File based**: `flare call --program [PROGRAM] --accounts-file [ACCOUNTS FILE PATH] --idl [IDL] [METHOD NAME] [ARGS]`

Where the `ACCOUNTS FILE PATH` points to a JSON file with the following format:
```json
{
    "addresses": { 
        "[account 1]": "5JUr...",
        "[account 2]": "9tLN...",
        "...": "..."
        "signer": "Cbt..."
    },
    "signers": { 
        "[account 1]": "account_1.json",
        "[signer]": "signer.json"
    }
}
```

`"addresses"` are the account addresses the program method requires, and `"signers"` are the file paths to the keypairs for the accounts marked as signers.

**Example**: 
```bash
./flare call --keypair keypair.json \
             --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve \
             --accounts-file ./example/example-accounts.json
             --idl ./example/onchain_voting.json \
              gibVote GM
```

If the `IDL` file path is not provided, Flare will try to fetch the published IDL.

#### Read Account
`flare read-account --program [PROGRAM] --idl [IDL] --account [ACCOUNT]`: Reads and deserializes the `ACCOUNT` with the provided address from the `PROGRAM`. If the `IDL` file path is not provided, Flare will try to fetch the published IDL.

**Example**: `flare --cluster devnet read-account --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve --account 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA --idl ./example/onchain_voting.json`

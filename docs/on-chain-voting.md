# On chain voting

In this document, we will show how a developer can use Flare to test and interact with the **On chain voting** example Anchor program. You can find more information about the program in its [Anchor by Example tutorial](https://examples.anchor-lang.com/docs/onchain-voting).

For documentation of all of Flare's commands, see [here](./README.md).

## Set-up

We assume Flare was installed an in the system path (or a binary is present in the working directory), see more information [here](../README.md). We also assume the project was deployed to devnet as indicated in the Anchor by Example tutorial, and that a keypair with devnet SOL balance is available (we will refer to this file as `keypair.json`).

For convenience, here is the address of the program in devnet, but do replace this with your own deployment address:

- `on chain voting (devnet)`: 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa

## IDL

If the project was built with Anchor, the IDL would already be available in the `target/idl` folder. If you wish to use the deployment we provide, you can get the IDL using Flare with:

```bash
flare fetch-idl --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa
```

**Note**: The IDL contains the program address, so make sure you are using the IDL from the specific program deployment, otherwise there might be some unexpected behaviour.

We will assume this IDL is present as `idl.json`.

## Calling the initVoteBank method

From the IDL:
```json
    {
      "name": "initVoteBank",
      "accounts": [
        {
          "name": "voteAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
```
Looking at the IDL, we can see the method requires three accounts, `voteAccount`, `signer`, and `systemProgram` (`11111111111111111111111111111111`)

Two are marked as signers; the main signer (which will be our wallet), and a `voteAccount`.


### Creating the `voteAccount`

We first need to create the `voteAccount` that will hold our vote count. We can do this by creating a wallet with Flare:

```bash
flare wallet-create
```
We get an output like:
```
Address: 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr
Mnemonic: admit peace apple garbage pony asset radio taste bitter art fabric utility

Wrote keypair to 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr.json
```
(your specific address and mnemonic will be different)

`5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr` will now be the address of our Vote Bank, and its keypair will be stored at `5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr.json`.

### Method call

We need to provide Flare with the following information:
- Address of the program (and cluster)
- Keypair of the wallet submitting the transaction
- Addresses of the accounts the method needs
- Keypairs of the signers

Flare allows two equivalent ways of calling methods. One is inline, in which we provide all the arguments via the CLI, and the other is with an *accounts file*. Let's look at both ways of doing it (which one you choose is up to you):

#### Inline

We can call the method with the following command:
```bash
flare --cluster devnet call --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa \
      --keypair keypair.json \
      --idl idl.json \
      --accounts 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr,(your wallet address),11111111111111111111111111111111 \
      --signers 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr.json,keypair.json
      -- initVoteBank
```

Where:
- `--cluster devnet` means we're working in devnet
- `--program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa` is the address of the program deployment
- `--keypair keypair.json` is our wallet keypair
- `--accounts 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr,(your wallet address),11111111111111111111111111111111` are the accounts needed by the method call
- `--signers 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr.json,keypair.json` point to the JSON files with the private keys of the signers
- `--` separates the program options from the method name and arguments (in this case, there are no arguments). This is not always necessary.
- `initVoteBank` is the method name

#### Using an accounts file
We can create a JSON file with the following format, and save it as `accounts.json`:
```json
{
  "addresses": {
    "voteAccount": "5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr",
    "signer": "(your wallet address)",
    "systemProgram": "11111111111111111111111111111111"
  },
  "signers": {
    "voteAccount": "5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr.json",
    "signer": "keypair.json"
  }
}
```

Now, we can call
```bash
flare --cluster devnet call --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa \
      --keypair keypair.json \
      --idl idl.json \
      --accounts-file accounts.json 
      -- initVoteBank

```

After this, we should have initiliazed the Vote Bank account.

## Calling the gibVote method

Again, we start by looking at the IDL:
```json
    {
      "name": "gibVote",
      "accounts": [
        {
          "name": "voteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "voteType",
          "type": {
            "defined": "VoteType"
          }
        }
      ]
    }

```

We can see that now we have only one signer. As this is the same account that sends the transaction, we can ommit the `--signers` argument. We now have one argument, the `voteType`, which is of type `VoteType` defined below in the IDL:

```json
  "types": [
    {
      "name": "VoteType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "GM"
          },
          {
            "name": "GN"
          }
        ]
      }
    }
  ],
```

Flare allows you to specify this argument by just saying either `GM` or `GN`.

Thus, the full command for voting `GM` will be:
```bash
flare --cluster devnet call \
        --keypair keypair.json \
        --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa \
        --accounts 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr,(your wallet address) \
        --idl idl.json \
         gibVote GM
```

## Reading from the account

Now, we want to see the updated vote count. For this, we need to read the Vote Bank account. Flare allows us to do this easily, by automatically infering the account type from the discriminator. We do not have to worry about that, and can just call:
```bash
flare --cluster devnet read-account \
      --program 39EmHuEbqkzUvPncNchXW1Yt6VPmps2Z9ucR82EozNAa \
      --account 5aWatcUZtvzfXnsyHN71JqiTgX4gBuqhED4aCRuLBVHr
```

If everything went well, we should get something like this:
```json
{"isOpenToVote":true,"gm":1,"gn":0}
```
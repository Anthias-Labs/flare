**:construction: WIP: This tool is in active development, and can experience breaking changes. For safety, it currently operates on Devnet by default**

# Flare

Flare is the Command Line Interface for developers on Solana. Below is a breakdown of the various commands included within Flare. If you run into any issues while operating Flare, please open an issue on this repo or contact a contributor from Anthias Labs [here](https://discord.com/invite/RCJYpMvkBy). 

## Installation
Please make sure you have Cargo in your system.
1. Clone the repo with `git clone https://github.com/Anthias-Labs/flare.git`
2. Move into the folder: `cd flare`
3. Build the project: `cargo build`
4. Move into the binary location `cd target/debug`
5. Run the compiled binary with `./flare`

## Usage
To test the tool and see example usage:
1. Write a mnemonic with some balance in devnet to a `.mnemonic` file in root.
2. Run `./example-usage.sh`. This will build the tool and test every subcommand.

## Functions
### Chain and Transaction Interactions:
`flare send`: allows sending SOL to an account

`flare balance`: checks SOL balance for a given address

`flare epoch`: get the current epoch number

`flare block-height`: get the current epoch number

`flare call`: calls an arbitrary Anchor program method

`flare read-account`: reads and deserializes an arbitrary account

`flare fetch-idl`: fetches the published IDL from an Anchor program


### Wallet Management:
`flare wallet-create`: generates keypair and mnemonic for a new wallet

`flare wallet-recover`: gets keypair from a given mnemonic

### Utils (Additional):
`flare sign`: sign an arbitrary message with a given private key

`flare address-derive`: derives address from a keypair file

`flare generate-pda`: generates Program Derived Address from program address and seeds

## Roadmap
- [X] Reading wallet SOL balance from chain
- [X] Reading epoch/block height
- [X] Sending SOL transactions between accounts
- [X] Creating wallets and recovering from mnemonic
- [X] Signing arbitrary messages
- [X] Adding CLI options for handling different cluster RPCs
- [X] Calling methods and reading state from on-chain programs
- [X] Adding utils for common operations
- [X] File-based  wallet and config management (storage and read)
- [X] Add support for multiple signers on an instruction
- [X] Add support for Program Derived Addresses
- [ ] Improve documentation and error messages

## About
Flare is the first tool from Solstice, the toolkit for developers on Solana. The development of Flare was originally funded by a grant from the Solana Foundation to Anthias Labs in January of 2024. Flare’s public launch is scheduled for April of 2024.


Anthias Labs is a boutique blockchain r&d firm focused on public goods tooling and actionable research. To see more about us, feel free to review [here](https://www.anthias.xyz/home).

## Acknowledgements
Acknowledgments of the contributors to Flare will be posted here once Flare v0 is shipped in late March of 2024

## Contributing
To contribute to Flare, please reach out to the current contributors in our main [Contributor Discord](https://discord.gg/RCJYpMvkBy). 


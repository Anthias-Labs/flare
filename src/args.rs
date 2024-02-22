use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlareCli {
    #[clap(subcommand)]
    pub command: FlareCommand,
}

#[derive(Debug, Subcommand)]
pub enum FlareCommand {
    /// Gets balance for a given wallet address
    Balance(BalanceCommand),

    /// Gets the current block height
    BlockHeight,

    /// Gets the current epoch
    Epoch,

    /// Sends lamport to another account
    Send(SendCommand),

    /// Signs an arbitrary message with a private key
    Sign(SignCommand),

    /// Creates a new random wallet
    WalletCreate,

    /// Recovers a wallet from its mnemonic
    WalletRecover(WalletRecoverCommand),
}

#[derive(Debug, Args)]
pub struct BalanceCommand {
    /// Pubkey
    pub pubkey: String,
}

#[derive(Debug, Args)]
pub struct SendCommand {
    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: String,

    /// Target pubkey
    #[arg(short, long)]
    pub to: String,

    /// Amount
    pub amount: u64,
}

#[derive(Debug, Args)]
pub struct SignCommand {
    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: String,

    /// Message
    pub msg: String,
}

#[derive(Debug, Args)]
pub struct WalletRecoverCommand {
    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: String,
}

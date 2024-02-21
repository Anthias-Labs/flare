use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlareCli {
    #[clap(subcommand)]
    pub command: FlareCommand,
}

#[derive(Debug, Subcommand)]
pub enum FlareCommand {
    /// Gets balance for a determined pubkey
    Balance(BalanceCommand),

    /// Gets block height
    BlockHeight,

    /// Gets epoch
    Epoch,

    /// Sends money
    Send(SendCommand),

    /// Signs message
    Sign(SignCommand),

    /// Creates a wallet
    WalletCreate,

    /// Recovers a wallet
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

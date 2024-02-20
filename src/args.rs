use clap::{Args, Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlareCli {
    #[clap(subcommand)]
    command: FlareCommand,
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
    pubkey: Pubkey,
}

#[derive(Debug, Args)]
pub struct SendCommand {
    /// Mnemonic
    #[arg(short, long)]
    mnemonic: String,

    /// Target pubkey
    #[arg(short, long)]
    to: Pubkey,

    /// Amount
    amount: f32,
}

#[derive(Debug, Args)]
pub struct SignCommand {
    /// Mnemonic
    #[arg(short, long)]
    mnemonic: String,

    /// Message
    msg: String,
}

#[derive(Debug, Args)]
pub struct WalletRecoverCommand {
    /// Mnemonic
    #[arg(short, long)]
    mnemonic: String,
}

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlareCli {
    #[clap(subcommand)]
    pub command: FlareCommand,

    /// Sets cluster (can be devnet, mainnet, testnet or a specific url)
    #[arg(short, long, default_value_t = String::from("devnet"))]
    pub cluster: String,
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

    /// Call to a program
    Call(CallCommand),

    /// Read account from program
    ReadAccount(ReadAccountCommand),
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

#[derive(Debug, Args)]
pub struct CallCommand {
    /// Program pubkey
    #[arg(short, long)]
    pub program: String,

    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: String,

    /// Instruction name
    pub instruction_name: String,

    /// Arguments separated by comma
    #[clap(value_delimiter = ',', num_args = 0..)]
    pub args: Vec<String>,

    // Account pubkeys separated by comma
    #[arg(short, long)]
    #[clap(value_delimiter = ',', num_args = 1..)]
    pub accounts: Vec<String>,

    /// Idl file path
    #[arg(short, long)]
    pub idl: String,
}

#[derive(Debug, Args)]
pub struct ReadAccountCommand {
    /// Program pubkey
    #[arg(short, long)]
    pub program: String,

    /// Account pubkey
    #[arg(short, long)]
    pub account: String,

    /// Idl file path
    #[arg(short, long)]
    pub idl: String,
}

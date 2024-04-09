use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlareCli {
    #[clap(subcommand)]
    pub command: FlareCommand,

    /// Sets cluster (can be devnet, mainnet, testnet or a specific url)
    #[arg(short, long, default_value_t = String::from("mainnet"))]
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

    /// Derives the address from a keypair file
    AddressDerive(AddressDeriveCommand),

    /// Call to a program
    Call(CallCommand),

    /// Read account from program
    ReadAccount(ReadAccountCommand),

    /// Fetch IDL from program
    FetchIDL(FetchIDLCommand),
}

#[derive(Debug, Args)]
pub struct BalanceCommand {
    /// Pubkey
    pub pubkey: String,
}

#[derive(Debug, Args)]
pub struct SendCommand {
    /// Keypair file
    #[arg(short, long)]
    pub keypair: Option<String>,

    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: Option<String>,

    /// Target pubkey
    #[arg(short, long)]
    pub to: String,

    /// Amount
    pub amount: u64,
}

#[derive(Debug, Args)]
pub struct SignCommand {
    /// Keypair file
    #[arg(short, long)]
    pub keypair: Option<String>,

    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: Option<String>,

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
    /// Program address
    #[arg(short, long)]
    pub program: String,

    /// Keypair file
    #[arg(short, long)]
    pub keypair: Option<String>,

    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: Option<String>,

    /// Instruction name
    pub instruction_name: String,

    /// Account pubkeys separated by comma
    #[arg(short, long)]
    #[clap(required_unless_present = "accounts_file", value_delimiter = ',', num_args = 1..)]
    pub accounts: Option<Vec<String>>,

    /// Signers
    #[arg(short, long)]
    #[clap(required_unless_present = "accounts_file", value_delimiter = ',', num_args = 1..)]
    pub signers: Option<Vec<String>>,

    /// Accounts file
    #[arg(short = 'f', long)]
    #[clap(
        required_unless_present = "accounts",
        required_unless_present = "signers"
    )]
    pub accounts_file: Option<String>,

    /// Arguments separated by comma
    #[clap(value_delimiter = ',', num_args = 0..)]
    pub args: Vec<String>,

    /// Idl file path
    #[arg(short, long)]
    pub idl: String,
}

#[derive(Debug, Args)]
pub struct ReadAccountCommand {
    /// Program address
    #[arg(short, long)]
    pub program: String,

    /// Account pubkey
    #[arg(short, long)]
    pub account: String,

    /// Idl file path
    #[arg(short, long)]
    pub idl: String,
}

#[derive(Debug, Args)]
pub struct FetchIDLCommand {
    /// Program address
    #[arg(short, long)]
    pub program: String,
}

#[derive(Debug, Args)]
pub struct AddressDeriveCommand {
    /// Keypair file
    #[arg(short, long)]
    pub keypair: String,
}

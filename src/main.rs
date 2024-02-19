mod lib;
use lib::{Context, wallet_from_seed_phrase, sign_message, new_wallet};

use anyhow::{Error, Result};
use std::str::FromStr;

use bip39::Mnemonic;
use rand::RngCore;
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{
    signature::{keypair_from_seed_phrase_and_passphrase, Keypair},
    signer::Signer,
    system_transaction,
};

use anchor_client::{Client, ClientError, Program};
use anchor_lang;
const URL: &str = "https://api.mainnet-beta.solana.com";

const URL_DEVNET: &str = "https://api.devnet.solana.com";
const URL_TESTNET: &str = "https://api.testnet.solana.com";

const MNEMONIC: &str = "mirror dry jazz old argue smooth jacket universe minimum latin text love";
const MNEMONIC_2: &str = "gift runway carpet cool scale trim beauty company hold beach visa festival";

fn main() -> Result<()> {
    let ctx = Context::new(URL);
    let pubkey = Pubkey::from_str("mrgn3H4uBbKAWBjdFKSGks3SpLm4q8YaRxUCMGa5ZBY").unwrap();
    println!("{}", ctx.get_balance(&pubkey)?);
    let k = new_wallet()?;
    println!("{}", k.key_pair.pubkey());

    let nw = wallet_from_seed_phrase(&k.mnemonic)?;
    println!("{}", nw.key_pair.pubkey());

    let msg = "TEST";

    println!("{}", sign_message(&nw, "msg"));

    println!("Current block height {}", ctx.get_block_height()?);
    println!("Current epoch number {}", ctx.get_epoch_number()?);
    Ok(())
}

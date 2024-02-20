mod lib;
use lib::{new_wallet, sign_message, wallet_from_seed_phrase, Context};

mod args;
use args::FlareCli;

use anyhow::{Error, Result};
use clap::Parser;
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
const MNEMONIC_2: &str =
    "gift runway carpet cool scale trim beauty company hold beach visa festival";

fn main() -> Result<()> {
    /* let ctx = Context::new(URL);
        let pubkey = Pubkey::from_str("mrgn3H4uBbKAWBjdFKSGks3SpLm4q8YaRxUCMGa5ZBY").unwrap();
        println!("{}", ctx.get_balance(&pubkey)?);
        let k = new_wallet()?;
        println!("{}", k.key_pair.pubkey());

    fn stage_test_balance() -> Result<()> {
        let ctx = Context::new(URL_DEVNET);
        let w = wallet_from_seed_phrase(MNEMONIC)?;

        println!("Balance w1: {}", ctx.get_balance(&w.key_pair.pubkey())?);
        Ok(())
    }

    fn stage_test_transfer() -> Result<()> {
        let ctx = Context::new(URL_DEVNET);
        let w = wallet_from_seed_phrase(MNEMONIC)?;
        let w2 = wallet_from_seed_phrase(MNEMONIC_2)?;

        println!("{} -> {}", w.key_pair.pubkey(), w2.key_pair.pubkey());

        println!("Previous balance w1: {}", ctx.get_balance(&w.key_pair.pubkey())?);
        println!("Previous balance w2: {}", ctx.get_balance(&w2.key_pair.pubkey())?);

        println!("Sending...");

        ctx.transfer_sol(&w, &w2.key_pair.pubkey(), 100)?;

        println!("Balance w1: {}", ctx.get_balance(&w.key_pair.pubkey())?);
        println!("Balance w2: {}", ctx.get_balance(&w2.key_pair.pubkey())?);

        Ok(())
    }

    fn stage_test_height() -> Result<()> {
        let ctx = Context::new(URL_DEVNET);
        println!("Block height: {}", ctx.get_block_height()?);
        println!("Epoch: {}", ctx.get_epoch_number()?);

        Ok(())
    }

    fn stage_test_wallet_gen_load() -> Result<()> {
        let wal = new_wallet()?;
        println!("Generated wallet:");
        println!("Public key: {}", wal.key_pair.pubkey());
        println!("Mnemonic: {}", wal.mnemonic);

        println!("Regenerate from mnemonic: ");
        let wal_2 = wallet_from_seed_phrase(&wal.mnemonic)?;
        println!("Regenerated pubkey: {}", wal_2.key_pair.pubkey());

        Ok(())
    }

    fn stage_test_sign_message() -> Result<()> {
        let msg = "Hello Solana!";
        let w = wallet_from_seed_phrase(MNEMONIC)?;

        println!("Signing message '{}' with wallet {}...", msg, w.key_pair.pubkey());
        println!("Signature: {}", sign_message(&w, msg));

        Ok(())
    }
    fn main() -> Result<()> {
        println!("\n\t Testing wallet generation and regenerate from mnemonic...");
        println!("\t ===========================\n");

        stage_test_wallet_gen_load()?;

        println!("\n\t Testing message signature...");
        println!("\t ===========================\n");

        stage_test_sign_message()?;

        println!("\n\t Testing block height and epoch...");
        println!("\t ===========================\n");

        stage_test_height()?;

        println!("\n\t Testing wallet balance...");
        println!("\t ===========================\n");

        stage_test_balance()?;

        println!("\n\t Testing transfer...");
        println!("\t ===========================\n");

        stage_test_transfer()?;


        let msg = "TEST";

        println!("{}", sign_message(&nw, "msg"));

        println!("Current block height {}", ctx.get_block_height()?);
        println!("Current epoch number {}", ctx.get_epoch_number()?);*/
    let args = FlareCli::parse();
    Ok(())
}

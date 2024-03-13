mod lib;
use lib::{new_wallet, sign_message, wallet_from_seed_phrase, Context};

mod args;
use args::{FlareCli, FlareCommand};
// use borsh::io;
use std::io;
use borsh::{BorshSerialize};

use anchor_client::{Client, ClientError, Config, Program};
use anchor_lang::AnchorSerialize;
use anyhow::{Error, Result};
use bip39::Mnemonic;
use clap::Parser;
use rand::RngCore;
use solana_clap_utils::input_validators::is_pubkey;
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{
    signature::{keypair_from_seed_phrase_and_passphrase, Keypair},
    signer::Signer,
    system_transaction,
};
use std::str::FromStr;
const URL: &str = "https://api.mainnet-beta.solana.com";

const URL_DEVNET: &str = "https://api.devnet.solana.com";
const URL_TESTNET: &str = "https://api.testnet.solana.com";

const MNEMONIC: &str = "mirror dry jazz old argue smooth jacket universe minimum latin text love";
const MNEMONIC_2: &str =
    "gift runway carpet cool scale trim beauty company hold beach visa festival";

fn get_context_from_cluster(cluster: &str) -> Context {
    let cluster_url;
    match cluster {
        "devnet" => cluster_url = URL_DEVNET,
        "mainnet" => cluster_url = URL,
        "testnet" => cluster_url = URL_TESTNET,
        &_ => cluster_url = cluster,
    }
    Context::new(cluster_url)
}

fn test_program() -> Result<()> {
    let prog_addr = "2CQAxft3JDVfMgjW3T73hWYFym1UZZWmuhHgq3JmEYa1";
    let prog_pub = Pubkey::from_str(prog_addr)?;

    Ok(())
}

#[derive(AnchorSerialize)]
struct Pito {
    a: i32,
    b: i64
}

fn test_ser() -> Result<()> {
    let p = Pito{a: 1141832, b: 2};
    let v = p.try_to_vec()?;
    println!("{:?}", v);

    Ok(())
}

fn main() -> Result<()> {
    test_ser()
    /*
    let args = FlareCli::parse();
    let cluster = args.cluster.to_lowercase();

    let ctx = get_context_from_cluster(&cluster);
    match args.command {
        FlareCommand::Balance(balance_data) => {
            let pubkey = Pubkey::from_str(&balance_data.pubkey)?;
            println!("Balance: {}", ctx.get_balance(&pubkey)?);
        }
        FlareCommand::WalletCreate => {
            println!("{}", new_wallet()?);
        }
        FlareCommand::WalletRecover(wallet_recover_data) => {
            let wallet = wallet_from_seed_phrase(&wallet_recover_data.mnemonic)?;
            println!("{}", wallet);
        }
        FlareCommand::Send(send_data) => {
            let source_wallet = wallet_from_seed_phrase(&send_data.mnemonic)?;
            let target_pubkey = Pubkey::from_str(&send_data.to)?;
            ctx.transfer_sol(&source_wallet, &target_pubkey, send_data.amount)?;
        }
        FlareCommand::Sign(sign_data) => {
            let wallet = wallet_from_seed_phrase(&sign_data.mnemonic)?;
            println!("Signed message: {}", sign_message(&wallet, &sign_data.msg));
        }
        FlareCommand::BlockHeight => println!("Block height: {}", ctx.get_block_height()?),
        FlareCommand::Epoch => println!("Epoch number: {}", ctx.get_epoch_number()?),
    }

    Ok(())*/
}

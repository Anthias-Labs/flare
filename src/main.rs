mod lib;
use lib::{
    new_wallet, read_wallet_file, sign_message, wallet_from_seed_phrase, write_wallet_file,
    Context, Wallet,
};

mod idl;
mod program_executor;

mod args;
use args::{FlareCli, FlareCommand};

// use borsh::io;
//use borsh::*;
use borsh::{BorshDeserialize, BorshSerialize};
use program_executor::ProgramExecutor;
use serde_json::Value;
use solana_sdk::address_lookup_table::instruction;
use solana_sdk::instruction::AccountMeta;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};

use anchor_client;
use anchor_client::{Client, ClientError, Config, Program};
use anchor_lang::{accounts, AnchorDeserialize, AnchorSerialize};
use anyhow::{Error, Result};
use bip39::Mnemonic;
use clap::Parser;
use rand::RngCore;
use solana_clap_utils::input_validators::is_pubkey;
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{
    client,
    instruction::Instruction,
    signature::{keypair_from_seed_phrase_and_passphrase, Keypair},
    signer::Signer,
    system_transaction,
    transaction::Transaction,
};
use std::str::FromStr;
const URL: &str = "https://api.mainnet-beta.solana.com";

const URL_DEVNET: &str = "https://api.devnet.solana.com";
const URL_TESTNET: &str = "https://api.testnet.solana.com";

fn get_wallet(mnemonic: &Option<String>, path: &Option<String>) -> Result<Wallet> {
    let wallet = match (mnemonic, path) {
        (Some(_), Some(_)) => {
            println!("Arguments must provide either a mnemonic or a keypair path, not both");
            return Err(Error::msg("Both mnemonic and keypair provided"));
        }
        (None, None) => {
            println!("At least one of mnemonic or keypair must be provided");
            return Err(Error::msg("Neither mnemonic or keypair provided"));
        }
        (Some(m), None) => return wallet_from_seed_phrase(&m),
        (None, Some(p)) => return read_wallet_file(&p),
    };
}

fn main() -> Result<()> {
    let args = FlareCli::parse();
    let cluster = args.cluster.to_lowercase();

    let ctx = Context::from_cluster(&cluster); // Cambiar por Context::from_cluster(&cluster)
    match args.command {
        FlareCommand::Balance(balance_data) => {
            let pubkey = Pubkey::from_str(&balance_data.pubkey)?;
            println!("Balance: {}", ctx.get_balance(&pubkey)?);
        }
        FlareCommand::WalletCreate => {
            let nw = new_wallet()?;
            println!("{}", nw);
            let path = format!("{}.json", nw.key_pair.pubkey());
            write_wallet_file(&nw, &path)?;
            println!("Wrote keypair to {}", path);
        }
        FlareCommand::WalletRecover(wallet_recover_data) => {
            let wallet = wallet_from_seed_phrase(&wallet_recover_data.mnemonic)?;
            println!("{}", wallet);
            let path = format!("{}.json", wallet.key_pair.pubkey());
            write_wallet_file(&wallet, &path)?;
            println!("Wrote keypair to {}", path);
        }
        FlareCommand::Send(send_data) => {
            let source_wallet = get_wallet(&send_data.mnemonic, &send_data.keypair)?;
            let target_pubkey = Pubkey::from_str(&send_data.to)?;
            ctx.transfer_sol(&source_wallet, &target_pubkey, send_data.amount)?;
        }
        FlareCommand::Sign(sign_data) => {
            let wallet = get_wallet(&sign_data.mnemonic, &sign_data.keypair)?;
            println!("Signed message: {}", sign_message(&wallet, &sign_data.msg));
        }
        FlareCommand::BlockHeight => println!("Block height: {}", ctx.get_block_height()?),
        FlareCommand::Epoch => println!("Epoch number: {}", ctx.get_epoch_number()?),
        FlareCommand::Call(call_data) => {
            let prog_id = Pubkey::from_str(&call_data.program)?;
            let payer = get_wallet(&call_data.mnemonic, &call_data.keypair)?;
            let instruction_name = call_data.instruction_name;
            let args = call_data.args; // Esta lectura hay que cambiarla para no pasar signer dos veces
            let mut account_pubkeys: Vec<Pubkey> = Vec::new();
            let mut signers_keypairs: Vec<Keypair> = Vec::new();
            let idl_path = call_data.idl;
            let program_executor = ProgramExecutor::from_file_with_context(ctx, &idl_path);
            if let Some(accounts) = call_data.accounts {
                for pubkey_str in accounts {
                    account_pubkeys.push(Pubkey::from_str(&pubkey_str)?)
                }
                if let Some(signers) = call_data.signers {
                    // logic to read signers from CLI
                } else {
                    panic!("Missing signers");
                }
            } else if let Some(accounts_file) = call_data.accounts_file {
                let pubkeys_and_keypairs = program_executor
                    .get_account_and_signers_from_file_for_instruction(
                        &instruction_name,
                        accounts_file,
                    );
                account_pubkeys = pubkeys_and_keypairs.0;
                signers_keypairs = pubkeys_and_keypairs.1;
            } else {
                panic!("Missing accounts and signers");
            }
            let mut signers_refs: Vec<&Keypair> = Vec::new();
            signers_keypairs
                .iter()
                .for_each(|keypair| signers_refs.push(keypair));
            program_executor.run_instruction(
                prog_id,
                &payer,
                &signers_refs,
                &instruction_name,
                &account_pubkeys,
                args,
            );
        }
        FlareCommand::ReadAccount(read_account_data) => {
            let prog_id = Pubkey::from_str(&read_account_data.program)?;
            let account_pubkey = Pubkey::from_str(&read_account_data.account)?;
            let idl_path = read_account_data.idl;
            let program_executor = ProgramExecutor::from_file_with_context(ctx, &idl_path);
            println!(
                "{}",
                program_executor.fetch_account(&prog_id, &account_pubkey)?
            );
        }
        FlareCommand::FetchIDL(fetch_idl_data) => {
            let idl = ctx.get_idl(&fetch_idl_data.program)?;
            let json = serde_json::to_string_pretty(&idl)?;
            let path = format!("{}.json", &fetch_idl_data.program);
            println!("{}", json);
            let mut file = File::create(&path)?;
            file.write(json.as_bytes())?;
            println!("Wrote to {}", &path);
        }
        FlareCommand::AddressDerive(address_derive_data) => {
            let wallet = read_wallet_file(&address_derive_data.keypair)?;
            println!("{}", wallet.key_pair.pubkey());
        }
    }

    Ok(())
}

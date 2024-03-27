mod lib;
use lib::{new_wallet, sign_message, wallet_from_seed_phrase, Context};

mod idl;
mod program_executor;

mod args;
use args::{FlareCli, FlareCommand};

// use borsh::io;
//use borsh::*;
use borsh::{BorshDeserialize, BorshSerialize};
use program_executor::ProgramExecutor;
use solana_sdk::address_lookup_table::instruction;
use solana_sdk::instruction::AccountMeta;
use std::collections::HashMap;
use std::io;

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

const MNEMONIC: &str = "mirror dry jazz old argue smooth jacket universe minimum latin text love";
const MNEMONIC_2: &str =
    "gift runway carpet cool scale trim beauty company hold beach visa festival";
/*
fn test_program() -> Result<()> {
    let prog_addr = "2CQAxft3JDVfMgjW3T73hWYFym1UZZWmuhHgq3JmEYa1";
    let prog_pub = Pubkey::from_str(prog_addr)?;

    Ok(())
} */

#[derive(BorshDeserialize, BorshSerialize)]
struct Ix {
    vote_type: VoteType,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VoteType {
    Gm,
    Gn,
}

#[derive(BorshDeserialize, Debug)]
pub struct VoteBank {
    pub is_open_to_vote: bool,
    pub gm: u64,
    pub gn: u64,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_client::anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}

/* fn test_ser() -> Result<()> {
    let ctx = Context::new(URL_DEVNET);
    let w = wallet_from_seed_phrase(MNEMONIC)?;
    let ix = Ix {
        vote_type: VoteType::Gn,
    };

    // aca calculo el discriminator
    let disc = sighash("global", "gib_vote"); // global mepa que es lo mismo siempre, si el metodo se llama gibVote lo pasas a gib_vote.

    let ixBytes: Vec<u8> = Vec::new();
    //let ixBytes = ix.try_to_vec()?; // seria el borsh del instruction data

    let mut data: Vec<u8> = disc.to_vec();
    data.extend(ixBytes); // necesitas concatenar discriminator con el borsh de instruction data

    println!("DATA {:?}", data);

    let prog_id = Pubkey::from_str("WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve")?; // es uno de los contratos de prueba de anchor, te deja votar por gm o gn

    // este metodo te pide pasarle dos accounts, el bank (que seria como un argumento) y el signer
    let acc_bank_address = Pubkey::from_str("78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA")?;
    let acc_bank = AccountMeta::new(acc_bank_address, false);
    let acc_sig = AccountMeta::new(w.key_pair.pubkey(), true);

    // esta funcion te pide solo address, el borsh de todo
    let instruction = Instruction::new_with_bytes(prog_id, &data, vec![acc_bank, acc_sig]);

    println!("Instruction {:?}", instruction);

    let blockhash = ctx.rpc_client.get_latest_blockhash()?; // agarras blockhash fresco

    // construyo el transaction con el instruction que hice antes, pubkey y keypair de la wallet que manda la transaccion y el blockhash que agarras antes
    let mut tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&w.key_pair.pubkey()),
        &[&w.key_pair],
        blockhash,
    );

    println!("\nTX {:?}", tx);

    ctx.rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)?; // con esta function le agrega un iconito de carga mientras manda la transaccion

    // aca leo despues de actualizar:
    let r: VoteBank = ctx.read_account(&acc_bank_address)?; // por ahora uso el BorshDeserialize para leer pero para el final seguramente tengamos que hacer otra cosa

    println!("\nAfter {:?}", r);

    Ok(())
} */

fn main() -> Result<()> {
    //test_ser()

    /*let executor = ProgramExecutor::from_file("devnet", "data/onchain_voting.json");
    let mut args = Vec::new();
    args.push("GM".to_string());
    let w = wallet_from_seed_phrase(MNEMONIC)?;
    let mut account_pubkeys: Vec<Pubkey> = Vec::new();
    account_pubkeys.push(Pubkey::from_str(
        "78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA",
    )?);
    account_pubkeys.push(w.key_pair.pubkey());
    let prog_id = Pubkey::from_str("WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve").unwrap();
    executor.run_instruction(prog_id, w, "gibVote", &account_pubkeys, args)?;
    let account_read = executor
        .fetch_account(
            &prog_id,
            &Pubkey::from_str("78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA")?,
        )
        .unwrap();
    println!("{}", account_read);*/

    let args = FlareCli::parse();
    let cluster = args.cluster.to_lowercase();

    let ctx = Context::from_cluster(&cluster); // Cambiar por Context::from_cluster(&cluster)
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
        FlareCommand::Call(call_data) => {
            let prog_id = Pubkey::from_str(&call_data.program)?;
            let payer = wallet_from_seed_phrase(&call_data.mnemonic)?;
            let instruction_name = call_data.instruction_name;
            let args = call_data.args; // Esta lectura hay que cambiarla para no pasar signer dos veces
            let mut account_pubkeys = Vec::new();
            for pubkey_str in call_data.accounts {
                account_pubkeys.push(Pubkey::from_str(&pubkey_str)?)
            }
            let idl_path = call_data.idl;
            let program_executor = ProgramExecutor::from_file_with_context(ctx, &idl_path);
            program_executor.run_instruction(
                prog_id,
                payer,
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
    }

    Ok(())
}

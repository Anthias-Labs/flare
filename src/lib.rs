use std::str::FromStr;

use anyhow::{Error, Result};
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

pub struct Context {
    pub rpc_client: RpcClient,
}

#[derive(Debug)]
pub struct Wallet {
    pub key_pair: Keypair,
    pub mnemonic: String,
}

impl Context {
    pub fn new(url: &str) -> Self {
        let rpc_client = RpcClient::new(url);
        Self { rpc_client }
    }

    pub fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        let balance = self.rpc_client.get_balance(pubkey)?;
        Ok(balance)
    }

    pub fn get_airdrop(&self, receiver: &Wallet, amount_sol: f64) -> Result<()> {
        let sig = self.rpc_client.request_airdrop(
            &receiver.key_pair.pubkey(),
            (amount_sol * (LAMPORTS_PER_SOL as f64)).floor() as u64,
        )?;
        //let sig = self.rpc_client.request_airdrop(&receiver.key_pair.pubkey(), 10)?;

        let mut i = 0;
        loop {
            let confirmed = self.rpc_client.confirm_transaction(&sig)?;
            if confirmed || i > 500 {
                break;
            }
            i += 1;
        }

        Ok(())
    }

    pub fn transfer_sol(&self, sender: &Wallet, receiver: &Pubkey, lamport_amt: u64) -> Result<()> {
        let tx = system_transaction::transfer(
            &sender.key_pair,
            receiver,
            lamport_amt,
            self.rpc_client.get_latest_blockhash()?,
        );

        let sig = self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(())
    }

    pub fn get_block_height(&self) -> Result<u64> {
        let num = self.rpc_client.get_block_height()?;
        Ok(num)
    }

    pub fn get_epoch_number(&self) -> Result<u64> {
        let epoch = self.rpc_client.get_epoch_info()?;
        Ok(epoch.epoch)
    }

    pub fn get_idl(&self, program_address: &str) -> Result<()> {
        let acc = self
            .rpc_client
            .get_account(&Pubkey::from_str(program_address)?)?;


        //let data = acc.try_desirialize();
        Ok(())

    }
}

pub fn generate_entropy() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut entropy = [0u8; 16];
    rng.fill_bytes(&mut entropy);
    entropy
}

pub fn new_wallet() -> Result<Wallet> {
    let entropy = generate_entropy();
    let seed_phrase = Mnemonic::from_entropy(&entropy)?.to_string();
    return wallet_from_seed_phrase(&seed_phrase);
}

pub fn wallet_from_seed_phrase(seed_phrase: &str) -> Result<Wallet> {
    let kp = keypair_from_seed_phrase_and_passphrase(&seed_phrase, "");
    let kp = match kp {
        Ok(keypair) => keypair,
        Err(_) => return Err(Error::msg("Error decoding passphrase")),
    };
    return Ok(Wallet {
        key_pair: kp,
        mnemonic: seed_phrase.to_string(),
    });
}

pub fn sign_message(signer: &Wallet, message: &str) -> String {
    let sig = signer.key_pair.sign_message(message.as_bytes());
    return sig.to_string();
}
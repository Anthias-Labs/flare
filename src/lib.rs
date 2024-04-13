use core::fmt;
use std::result::Result::Ok;
use std::{fs, str::FromStr};

use anyhow::{Error, Result};
use bip39::Mnemonic;
use borsh::BorshDeserialize;
use rand::RngCore;
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{
        keypair_from_seed_phrase_and_passphrase, read_keypair_file, write_keypair_file, Keypair,
    },
    signer::Signer,
    system_transaction,
};

use anchor_lang::idl::IdlAccount;
use anchor_lang::AnchorDeserialize;
use anchor_syn::idl::types::Idl;
use flate2::read::ZlibDecoder;
use std::io::Read;

use serde_yml;
use std::collections::HashMap;

const URL: &str = "https://api.mainnet-beta.solana.com";

const URL_DEVNET: &str = "https://api.devnet.solana.com";
const URL_TESTNET: &str = "https://api.testnet.solana.com";

pub struct Context {
    pub rpc_client: RpcClient,
}

#[derive(Debug)]
pub struct Wallet {
    pub key_pair: Keypair,
    pub mnemonic: String,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address: {}\nMnemonic: {}\n",
            self.key_pair.pubkey(),
            self.mnemonic
        )
    }
}

impl Context {
    pub fn new(url: &str, finalized: bool) -> Self {
        let comm_scheme: CommitmentConfig;
        if finalized {
            comm_scheme = CommitmentConfig::finalized();
        } else {
            comm_scheme = CommitmentConfig::confirmed();
        }
        let rpc_client = RpcClient::new_with_commitment(url, comm_scheme);

        Self { rpc_client }
    }

    pub fn from_cluster(cluster: &str, finalized: bool) -> Context {
        let cluster_url;
        match cluster {
            "devnet" => cluster_url = URL_DEVNET,
            "mainnet" => cluster_url = URL,
            "testnet" => cluster_url = URL_TESTNET,
            &_ => cluster_url = cluster,
        }
        Context::new(cluster_url, finalized)
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

        self.rpc_client
            .send_and_confirm_transaction_with_spinner(&tx)?;
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

    pub fn get_idl(&self, program_address: &str) -> Result<Idl> {
        let idl_addr = Pubkey::from_str(&program_address)?;
        let mut account = self
            .rpc_client
            .get_account(&Pubkey::from_str(program_address)?)?;
        if account.executable {
            let idl_addr = IdlAccount::address(&idl_addr);
            account = self.rpc_client.get_account(&idl_addr)?;
        }

        // Cut off account discriminator.
        let mut d: &[u8] = &account.data[8..];
        let idl_account: IdlAccount = AnchorDeserialize::deserialize(&mut d)?;

        let compressed_len: usize = idl_account.data_len.try_into().unwrap();
        let compressed_bytes = &account.data[44..44 + compressed_len];
        let mut z = ZlibDecoder::new(compressed_bytes);
        let mut s = Vec::new();
        z.read_to_end(&mut s)?;
        serde_json::from_slice(&s[..]).map_err(Into::into)
    }

    pub fn read_account<T: BorshDeserialize>(&self, account_address: &Pubkey) -> Result<T> {
        let account = self.rpc_client.get_account(account_address)?;
        let mut data = &account.data[8..];
        let r: T = BorshDeserialize::deserialize(&mut data)?;
        Ok(r)
    }

    pub fn fetch_account(&self, account_address: &Pubkey) -> Result<Vec<u8>> {
        let account = self.rpc_client.get_account(account_address)?;
        let data = account.data.to_vec();
        Ok(data)
    }
}

pub fn try_get_default_rpc() -> Result<String> {
    let home = std::env::home_dir().unwrap();
    let config_path = home.join(".config/solana/install/config.yml");
    let config_raw = fs::read_to_string(config_path)?;
    let config: HashMap<String, serde_yml::Value> = serde_yml::from_str(&config_raw)?;
    let rpc = &config["json_rpc_url"];
    let rpc_string = rpc.as_str().unwrap();
    Ok(rpc_string.to_string())
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

pub fn read_wallet_file(path: &str) -> Result<Wallet> {
    let kp = read_keypair_file(path);
    let kp = match kp {
        Ok(keypair) => keypair,
        Err(_) => return Err(Error::msg("Error reading keypair file")),
    };

    return Ok(Wallet {
        key_pair: kp,
        mnemonic: "".to_string(),
    });
}

pub fn write_wallet_file(wallet: &Wallet, path: &str) -> Result<()> {
    let kp = &wallet.key_pair;
    let r = write_keypair_file(kp, path);
    match r {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::msg("Error writing keypair file")),
    }
}

pub fn sign_message(signer: &Wallet, message: &str) -> String {
    let sig = signer.key_pair.sign_message(message.as_bytes());
    return sig.to_string();
}

pub fn generate_pda_address(seeds: Vec<String>, program_id: &Pubkey) -> (Pubkey, u8) {
    let mut seeds_u8: Vec<Vec<u8>> = Vec::new();

    for seed in seeds {
        let pubkey_result = Pubkey::from_str(&seed);
        if let Ok(pubkey) = pubkey_result {
            seeds_u8.push(pubkey.to_bytes().to_vec());
            continue;
        }

        let num_parse_result = seed.parse::<i64>();
        if let Ok(num) = num_parse_result {
            seeds_u8.push(num.to_ne_bytes().to_vec());
            continue;
        }

        seeds_u8.push(seed.as_bytes().to_vec())
    }

    let seeds_u8_refs: Vec<&[u8]> = seeds_u8.iter().map(AsRef::as_ref).collect();

    Pubkey::find_program_address(&seeds_u8_refs, program_id)
}

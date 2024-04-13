use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use anchor_syn::idl::types::{
    Idl, IdlAccountItem, IdlInstruction, IdlType, IdlTypeDefinition, IdlTypeDefinitionTy,
};
use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use convert_case::{Case, Casing};
use serde_json::{Map, Value};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::lib::{read_wallet_file, Context, Wallet};

//use crate::idl::{Idl, IdlEnumType, IdlInstruction, IdlKind, IdlType};
use solana_program::pubkey::Pubkey;

/**
 * Implementar funciones para obtener
 * el borsh de argumentos primitivos.
 * Luego analizar como hacer con structs
 * y las demas posibilidades de tipos.
 */

macro_rules! serialize_by_type {
    ($to_serialize:expr, $to_save_serialize: expr, $t:ty) => {
        $to_serialize
            .parse::<$t>()
            .unwrap()
            .serialize($to_save_serialize);
    };
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

pub struct ProgramExecutor {
    idl: Idl,
    context: Context,
}

// CAMBIAR ESTO PARA QUE SE PUEDA INSTANCIAR POR FILE O POR GET_IDL
impl ProgramExecutor {
    fn get_idl_from_file(path: &str) -> Idl {
        serde_json::from_str(&fs::read_to_string(path).expect("Cant IDL (change message)"))
            .expect("Cant JSON (change this message)")
    }

    fn get_metadata() -> Map<String, Value> {
        let mut metadata = Map::new();
        metadata.insert("origin".to_string(), Value::String("anchor".to_string()));
        metadata
    }

    fn get_idl_string(&self) -> String {
        serde_json::to_string(&self.idl).unwrap()
    }

    pub fn from_file(cluster: &str, finalized: bool, path: &str) -> Self {
        ProgramExecutor::from_file_with_context(Context::from_cluster(cluster, finalized), path)
    }

    pub fn from_program_address(cluster: &str, finalized: bool, program_address: &str) -> Result<Self> {
        ProgramExecutor::from_program_address_with_context(
            Context::from_cluster(cluster, finalized),
            program_address,
        )
    }

    pub fn from_file_with_context(context: Context, path: &str) -> Self {
        let mut idl: Idl = ProgramExecutor::get_idl_from_file(path);
        idl.metadata = Some(Value::Object(ProgramExecutor::get_metadata()));
        ProgramExecutor { idl, context }
    }

    pub fn from_program_address_with_context(context: Context, program_address: &str) -> Result<Self> {
        let mut idl = context.get_idl(program_address)?;
        idl.metadata = Some(Value::Object(ProgramExecutor::get_metadata()));
        Ok(ProgramExecutor { idl, context })
    }
}

impl ProgramExecutor {
    fn get_serialized_args_rec(
        &self,
        type_to_serialize: IdlType,
        args: &Vec<String>,
        pos: usize,
    ) -> (Vec<u8>, usize) {
        let mut serialized_type: Vec<u8> = Vec::new();
        let mut pos = pos;
        match type_to_serialize {
            IdlType::Bool => {
                serialize_by_type!(args[pos], &mut serialized_type, bool);
                pos += 1;
            }
            IdlType::U8 => {
                serialize_by_type!(args[pos], &mut serialized_type, u8);
                pos += 1;
            }
            IdlType::I8 => {
                serialize_by_type!(args[pos], &mut serialized_type, i8);
                pos += 1;
            }
            IdlType::U16 => {
                serialize_by_type!(args[pos], &mut serialized_type, u16);
                pos += 1;
            }
            IdlType::I16 => {
                serialize_by_type!(args[pos], &mut serialized_type, i16);
                pos += 1;
            }
            IdlType::U32 => {
                serialize_by_type!(args[pos], &mut serialized_type, u32);
                pos += 1;
            }
            IdlType::I32 => {
                serialize_by_type!(args[pos], &mut serialized_type, i32);
                pos += 1;
            }
            IdlType::U64 => {
                serialize_by_type!(args[pos], &mut serialized_type, u64);
                pos += 1;
            }
            IdlType::I64 => {
                serialize_by_type!(args[pos], &mut serialized_type, i64);
                pos += 1;
            }
            IdlType::U128 => {
                serialize_by_type!(args[pos], &mut serialized_type, u128);
                pos += 1;
            }
            IdlType::I128 => {
                serialize_by_type!(args[pos], &mut serialized_type, i128);
                pos += 1;
            }
            IdlType::F32 => {
                serialize_by_type!(args[pos], &mut serialized_type, f32);
                pos += 1;
            }
            IdlType::F64 => {
                serialize_by_type!(args[pos], &mut serialized_type, f64);
                pos += 1;
            }
            IdlType::String => {
                serialize_by_type!(args[pos], &mut serialized_type, String);
                pos += 1;
            }
            IdlType::PublicKey => {
                serialize_by_type!(args[pos], &mut serialized_type, Pubkey);
                pos += 1;
            }
            IdlType::Bytes => {
                args[pos]
                    .as_bytes()
                    .serialize(&mut serialized_type)
                    .unwrap(); // chequear dif entre as_bytes y serialize
                pos += 1;
            }
            IdlType::Defined(defined_type_name) => {
                let defined_type = self.get_defined_type_by_name(defined_type_name).unwrap();
                let kind = defined_type.ty;
                match kind {
                    IdlTypeDefinitionTy::Struct { fields } => {
                        for field in fields.iter() {
                            let mut rec_call_to_type =
                                self.get_serialized_args_rec(field.ty.clone(), args, pos);
                            serialized_type.append(&mut rec_call_to_type.0);
                            pos = rec_call_to_type.1;
                        }
                    }
                    IdlTypeDefinitionTy::Enum { variants } => {
                        let mut count = 0;
                        for each_variant in variants.iter() {
                            if args[pos].eq(&each_variant.name) {
                                serialized_type.push(count);
                                break; // wacala
                            }
                            count += 1;
                        }
                        pos += 1;
                    }
                    IdlTypeDefinitionTy::Alias { value } => {
                        // que es esto?
                    }
                }
            }
            IdlType::Vec(vec_type) => {
                let arr_size = args[pos].parse::<u64>().unwrap();
                let mut counter = 0;
                pos += 1;
                while counter < arr_size {
                    let mut serialized_position =
                        self.get_serialized_args_rec(*vec_type.clone(), args, pos);
                    serialized_type.append(&mut serialized_position.0);
                    pos = serialized_position.1;
                    counter += 1;
                }
            }
            IdlType::Array(array_type, array_size) => {
                let mut counter = 0;
                pos += 1;
                while counter < array_size {
                    let mut serialized_position =
                        self.get_serialized_args_rec(*array_type.clone(), args, pos);
                    serialized_type.append(&mut serialized_position.0);
                    pos = serialized_position.1;
                    counter += 1;
                }
            }
            // ver como hacer estos y algunos ver que carajos jaja salu2
            IdlType::Option(option_type) => {}
            IdlType::GenericLenArray(array_type, string_rare) => {}
            IdlType::Generic(string_rare) => {}
            IdlType::DefinedWithTypeArgs { name, args } => {}
            IdlType::U256 => {}
            IdlType::I256 => {}
        }
        (serialized_type, pos)
    }

    fn get_defined_type_by_name(&self, type_name: String) -> Option<IdlTypeDefinition> {
        for idl_type in self.idl.types.iter() {
            if idl_type.name == type_name {
                return Some(idl_type.clone());
            }
        }
        None
    }

    fn get_serialized_args_for_instruction(
        &self,
        instruction: &IdlInstruction,
        args: Vec<String>,
    ) -> Vec<u8> {
        let mut pos: usize = 0;
        let mut args_serialized: Vec<u8> = Vec::new();
        for arg in instruction.args.iter() {
            let mut serialized_for_arg = self.get_serialized_args_rec(arg.ty.clone(), &args, pos);
            args_serialized.append(&mut serialized_for_arg.0);
            pos = serialized_for_arg.1;
        }
        args_serialized
    }

    fn get_accounts_for_instruction(
        &self,
        instruction: &IdlInstruction,
        account_pubkeys: &Vec<Pubkey>,
    ) -> Option<Vec<AccountMeta>> {
        let mut accounts: Vec<AccountMeta> = Vec::new();
        let mut it: usize = 0;
        for account in instruction.accounts.iter() {
            if let IdlAccountItem::IdlAccount(account) = account {
                if let Some(pubkey) = account_pubkeys.get(it) {
                    if account.is_mut {
                        accounts.push(AccountMeta::new(*pubkey, account.is_signer))
                    } else {
                        accounts.push(AccountMeta::new_readonly(*pubkey, account.is_signer))
                    }
                } else {
                    return None;
                }
            }
            it += 1;
        }
        return Some(accounts);
    }
}

impl ProgramExecutor {
    pub fn get_idl(&self) -> Idl {
        self.idl.clone()
    }

    pub fn get_instructions(&self) -> Vec<IdlInstruction> {
        self.idl.instructions.clone()
    }

    pub fn get_instruction_by_name(&self, instruction_name: &str) -> Option<IdlInstruction> {
        for instruction in self.idl.instructions.iter() {
            if instruction.name == instruction_name {
                return Some(instruction.clone());
            }
        }
        None
    }

    pub fn get_account_and_signers_from_file_for_instruction(
        &self,
        instruction_name: &String,
        path: String,
    ) -> (Vec<Pubkey>, Vec<Keypair>) {
        let json: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let instruction = self.get_instruction_by_name(instruction_name).unwrap();
        let addresses = json["addresses"].clone();
        if addresses == Value::Null {
            panic!("Missing account addresses");
        }
        let signers = json["signers"].clone();
        let mut pubkeys: Vec<Pubkey> = Vec::new();
        let mut keypairs: Vec<Keypair> = Vec::new();
        let mut signers_flag = false;
        for account in instruction.accounts.iter() {
            if let IdlAccountItem::IdlAccount(account) = account {
                let name = &account.name;
                let address = addresses[name].clone();
                if let Value::String(address) = address {
                    pubkeys.push(Pubkey::from_str(&address).unwrap());
                } else {
                    panic!("Account address must be a String");
                }
                if account.is_signer {
                    if !signers_flag {
                        if signers == Value::Null {
                            panic!("Missing signers");
                        }
                        signers_flag = true;
                    }
                    let signer = signers[name].clone();
                    if let Value::String(keypair_file) = signer {
                        let wallet = read_wallet_file(&keypair_file).unwrap();
                        keypairs.push(wallet.key_pair);
                    } else {
                        panic!("Signer keypair file must be a String");
                    }
                }
            }
        }
        (pubkeys, keypairs)
    }

    pub fn run_instruction(
        &self,
        prog_id: Pubkey,
        payer: &Wallet,
        signers: &Vec<&Keypair>,
        instruction_name: &str,
        account_pubkeys: &Vec<Pubkey>,
        args: Vec<String>,
    ) -> Result<()> {
        let instruction = self.get_instruction_by_name(instruction_name).unwrap();
        let serialized_args = self.get_serialized_args_for_instruction(&instruction, args);
        let mut data = sighash("global", &instruction_name.to_case(Case::Snake)).to_vec();
        data.extend(serialized_args);
        let accounts = self
            .get_accounts_for_instruction(&instruction, &account_pubkeys)
            .unwrap();
        let instruction = Instruction::new_with_bytes(prog_id, &data, accounts);
        let blockhash = self.context.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.key_pair.pubkey()),
            signers,
            blockhash,
        );
        //println!("\nTX {:?}", tx);
        self.context
            .rpc_client
            .send_and_confirm_transaction_with_spinner(&tx)?;

        Ok(())
    }

    pub fn read_account<T: BorshDeserialize>(&self, account_pubkey: &Pubkey) -> Result<T> {
        self.context.read_account(account_pubkey) // por ahora uso el BorshDeserialize para leer pero para el final seguramente tengamos que hacer otra cosa
    }

    pub fn fetch_account(&self, prog_id: &Pubkey, account_pubkey: &Pubkey) -> Result<String> {
        let idl_string = self.get_idl_string();
        let opts = sol_chainsaw::JsonSerializationOpts {
            pubkey_as_base58: true,
            n64_as_string: false,
            n128_as_string: true,
        };
        let mut chainsaw = sol_chainsaw::ChainsawDeserializer::new(&opts);
        chainsaw.add_idl_json(
            prog_id.to_string(),
            &idl_string,
            sol_chainsaw::IdlProvider::Anchor,
        )?;
        let acc_data = self.context.fetch_account(&account_pubkey)?;
        let mut acc_data_slice: &[u8] = &acc_data;
        Ok(chainsaw
            .deserialize_account_to_json_string(&prog_id.to_string(), &mut acc_data_slice)?)
    }
}

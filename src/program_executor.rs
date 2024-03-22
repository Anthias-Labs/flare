use std::collections::HashMap;
use std::fs;

use anchor_syn::idl::types::{
    Idl, IdlAccountItem, IdlInstruction, IdlType, IdlTypeDefinition, IdlTypeDefinitionTy,
};
use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use convert_case::{Case, Casing};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::sighash;

use crate::lib::{Context, Wallet};

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

pub struct ProgramExecutor {
    idl: Idl,
    context: Context,
}

// CAMBIAR ESTO PARA QUE SE PUEDA INSTANCIAR POR FILE O POR GET_IDL
impl ProgramExecutor {
    pub fn from_file(cluster: &str, path: &str) -> Self {
        ProgramExecutor {
            idl: serde_json::from_str(
                &fs::read_to_string(path).expect("Cant IDL (change message)"),
            )
            .expect("Cant JSON (change this message)"),
            context: Context::from_cluster(cluster),
        }
    }

    pub fn from_program_address(cluster: &str, program_address: &str) -> Self {
        let context = Context::from_cluster(cluster);
        ProgramExecutor {
            idl: context.get_idl(program_address).unwrap(),
            context,
        }
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
        account_pubkeys: &HashMap<String, Pubkey>,
    ) -> Option<Vec<AccountMeta>> {
        let mut accounts: Vec<AccountMeta> = Vec::new();
        for account in instruction.accounts.iter() {
            if let IdlAccountItem::IdlAccount(account) = account {
                if let Some(pubkey) = account_pubkeys.get(&account.name) {
                    if account.is_mut {
                        accounts.push(AccountMeta::new(*pubkey, account.is_signer))
                    } else {
                        accounts.push(AccountMeta::new_readonly(*pubkey, account.is_signer))
                    }
                } else {
                    return None;
                }
            }
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

    pub fn run_instruction(
        &self,
        prog_id: Pubkey,
        payer: Wallet,
        instruction_name: &str,
        account_pubkeys: &HashMap<String, Pubkey>,
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
            &[&payer.key_pair],
            blockhash,
        );
        println!("\nTX {:?}", tx);
        self.context
            .rpc_client
            .send_and_confirm_transaction_with_spinner(&tx)?;

        Ok(())
    }

    pub fn read_account<T: BorshDeserialize>(&self, account_pubkey: &Pubkey) -> Result<T> {
        self.context.read_account(account_pubkey) // por ahora uso el BorshDeserialize para leer pero para el final seguramente tengamos que hacer otra cosa
    }
}

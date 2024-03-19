use std::fs;

use anyhow::Result;
use borsh::BorshSerialize;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::{sighash, VoteBank};

use crate::lib::Context;

use crate::idl::{Idl, IdlEnumType, IdlInstruction, IdlKind, IdlType};
use flare::wallet_from_seed_phrase;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

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
}

impl From<&str> for ProgramExecutor {
    fn from(value: &str) -> Self {
        ProgramExecutor {
            idl: serde_json::from_str(
                &fs::read_to_string(value).expect("Cant IDL (change message)"),
            )
            .expect("Cant JSON (change this message)"),
        }
    }
}

impl ProgramExecutor {
    fn get_serialized_args_for_instruction_rec(
        &self,
        type_to_serialize: IdlEnumType,
        args: &Vec<String>,
        pos: usize,
    ) -> (Vec<u8>, usize) {
        let mut serialized_type: Vec<u8> = Vec::new();
        let mut pos = pos;
        match type_to_serialize {
            IdlEnumType::bool => {
                serialize_by_type!(args[pos], &mut serialized_type, bool);
                pos += 1;
            }
            IdlEnumType::u8 => {
                serialize_by_type!(args[pos], &mut serialized_type, u8);
                pos += 1;
            }
            IdlEnumType::i8 => {
                serialize_by_type!(args[pos], &mut serialized_type, i8);
                pos += 1;
            }
            IdlEnumType::u16 => {
                serialize_by_type!(args[pos], &mut serialized_type, u16);
                pos += 1;
            }
            IdlEnumType::i16 => {
                serialize_by_type!(args[pos], &mut serialized_type, i16);
                pos += 1;
            }
            IdlEnumType::u32 => {
                serialize_by_type!(args[pos], &mut serialized_type, u32);
                pos += 1;
            }
            IdlEnumType::i32 => {
                serialize_by_type!(args[pos], &mut serialized_type, i32);
                pos += 1;
            }
            IdlEnumType::u64 => {
                serialize_by_type!(args[pos], &mut serialized_type, u64);
                pos += 1;
            }
            IdlEnumType::i64 => {
                serialize_by_type!(args[pos], &mut serialized_type, i64);
                pos += 1;
            }
            IdlEnumType::u128 => {
                serialize_by_type!(args[pos], &mut serialized_type, u128);
                pos += 1;
            }
            IdlEnumType::i128 => {
                serialize_by_type!(args[pos], &mut serialized_type, i128);
                pos += 1;
            }
            IdlEnumType::f32 => {
                serialize_by_type!(args[pos], &mut serialized_type, f32);
                pos += 1;
            }
            IdlEnumType::f64 => {
                serialize_by_type!(args[pos], &mut serialized_type, f64);
                pos += 1;
            }
            IdlEnumType::string => {
                serialize_by_type!(args[pos], &mut serialized_type, String);
                pos += 1;
            }
            IdlEnumType::publicKey => {
                serialize_by_type!(args[pos], &mut serialized_type, Pubkey);
                pos += 1;
            }
            IdlEnumType::bytes => {
                args[pos].as_bytes().serialize(&mut serialized_type); // chequear dif entre as_bytes y serialize
                pos += 1;
            }
            IdlEnumType::defined(defined_type_name) => {
                let defined_type = self.get_defined_type_by_name(defined_type_name).unwrap();
                let kind = defined_type.ty.kind;
                match kind {
                    IdlKind::Struct => {
                        for field in defined_type.ty.fields.unwrap().iter() {
                            let mut rec_call_to_type = self
                                .get_serialized_args_for_instruction_rec(
                                    field.ty.clone(),
                                    args,
                                    pos,
                                );
                            serialized_type.append(&mut rec_call_to_type.0);
                            pos = rec_call_to_type.1;
                        }
                    }
                    IdlKind::Enum => {
                        let mut count = 0;
                        for each_variant in defined_type.ty.variants.unwrap().iter() {
                            if args[pos].eq(&each_variant.name) {
                                serialized_type.push(count);
                                break; // wacala
                            }
                            count += 1;
                        }
                        pos += 1;
                    }
                }
            }
            IdlEnumType::vec(vec_type) => {
                let arr_size = args[pos].parse::<u64>().unwrap();
                let mut counter = 0;
                pos += 1;
                while counter < arr_size {
                    let mut serialized_position =
                        self.get_serialized_args_for_instruction_rec(*vec_type.clone(), args, pos);
                    serialized_type.append(&mut serialized_position.0);
                    pos = serialized_position.1;
                    counter += 1;
                }
            }
            IdlEnumType::array(array_type, array_size) => {
                let mut counter = 0;
                pos += 1;
                while counter < array_size {
                    let mut serialized_position = self.get_serialized_args_for_instruction_rec(
                        *array_type.clone(),
                        args,
                        pos,
                    );
                    serialized_type.append(&mut serialized_position.0);
                    pos = serialized_position.1;
                    counter += 1;
                }
            }
            IdlEnumType::option(option_type) => {}
        }
        (serialized_type, pos)
    }

    fn get_defined_type_by_name(&self, type_name: String) -> Option<IdlType> {
        for idl_type in self.idl.types.iter() {
            if idl_type.name == type_name {
                return Some(idl_type.clone());
            }
        }
        None
    }

    fn get_serialized_args_for_instruction(
        &self,
        instruction_name: String,
        args: Vec<String>,
    ) -> Vec<u8> {
        let instruction = self.get_instruction_by_name(instruction_name).unwrap();
        let mut pos: usize = 0;
        let mut args_serialized: Vec<u8> = Vec::new();
        for arg in instruction.args.iter() {
            let mut serialized_for_arg =
                self.get_serialized_args_for_instruction_rec(arg.ty.clone(), &args, pos);
            args_serialized.append(&mut serialized_for_arg.0);
            pos = serialized_for_arg.1;
        }
        args_serialized
    }
}

impl ProgramExecutor {
    pub fn get_idl(&self) -> Idl {
        self.idl.clone()
    }

    pub fn get_instructions(&self) -> Vec<IdlInstruction> {
        self.idl.instructions.clone()
    }

    pub fn get_instruction_by_name(&self, instruction_name: String) -> Option<IdlInstruction> {
        for instruction in self.idl.instructions.iter() {
            if instruction.name == instruction_name {
                return Some(instruction.clone());
            }
        }
        None
    }

    pub fn run_instruction(&self, instruction_name: String, args: Vec<String>) -> Result<()> {
        let serialized_args = self.get_serialized_args_for_instruction(instruction_name, args);

        const URL_DEVNET: &str = "https://api.devnet.solana.com";
        const MNEMONIC: &str =
            "mirror dry jazz old argue smooth jacket universe minimum latin text love";
        let ctx = Context::new(URL_DEVNET);
        let w = wallet_from_seed_phrase(MNEMONIC)?;

        let mut data = sighash("global", "gib_vote").to_vec();
        data.extend(serialized_args);

        println!("{:?}", data);

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
    }
}

/* fn main() -> Result<()> {
    let program_executor = ProgramExecutor::from("./onchain_voting.json");

    let mut args = Vec::new();
    args.push("GM".to_string());
    program_executor.run_instruction("gibVote".to_string(), args);
    //println!("{:?}", program_executor.get_instructions());

    Ok(())
} */

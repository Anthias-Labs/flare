use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Idl {
    pub version: String,
    pub name: String,
    pub instructions: Vec<IdlInstruction>,
    pub types: Vec<IdlType>,
    pub accounts: Vec<IdlType>,
    pub errors: Option<Vec<IdlError>>,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlAccount>,
    pub args: Vec<IdlField>,
    pub returns: Option<IdlEnumType>,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlAccount {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlEnumType,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub enum IdlEnumType {
    bool,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    u64,
    i64,
    f64,
    u128,
    i128,
    bytes,
    string,
    publicKey,
    defined(String),
    option(Box<IdlEnumType>),
    vec(Box<IdlEnumType>),
    array(Box<IdlEnumType>, usize),
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlType {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlKindType,
}

#[derive(Debug, Deserialize, BorshSerialize, BorshDeserialize, Clone)]
pub enum IdlKind {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "enum")]
    Enum,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlKindType {
    pub kind: IdlKind,
    pub fields: Option<Vec<IdlField>>,
    pub variants: Option<Vec<IdlEnumVariant>>,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlEnumVariant {
    pub name: String,
    //fields: Option<Vec<IdlVariantType>>,
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(untagged)]
pub enum IdlVariantType {
    String(String),
    Field(IdlField),
}

#[derive(Debug, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct IdlError {
    pub code: u32,
    pub name: String,
    pub msg: Option<String>,
}

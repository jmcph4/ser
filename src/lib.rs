#![allow(unused)]
// #![feature(adt_const_params)]
extern crate z3 as z3_ext;
pub mod instruction;
pub mod machine;
pub mod memory;
pub mod record;
pub mod smt;
pub mod stack;
pub mod state;
pub mod traits;
pub mod conversion;
pub mod storage;
pub mod parser;
use paste::{expr, item, paste};
use instruction::*;
use smt::*;
use stack::*;
use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};
use rand::Rng;


pub fn bvi<const SZ: u32>(val: impl Into<i32>) -> BitVec<SZ> {
    BitVec::new_literal(val.into() as u64)
}

pub fn bvi_32byte(val: u64) -> BitVec<32> {
    BitVec::new_literal(val)
}

pub fn bvi_8byte(val: u64) -> BitVec<8> {
    BitVec::new_literal(val)
}
pub fn bvc(val: impl AsRef<str>) -> BitVec<32> {
    BitVec::new_const(val)
}
pub fn random_bv_arg<const SZ: u32>() -> BitVec<SZ> {
    let mut rng = rand::thread_rng();
    let rand_num: u64 = rng.gen();
    BitVec::new_literal(rand_num)
}

#[cfg(test)]
mod test {
    /*


     */
    pub const SIMPLE_COUNTER: &str = r"6080604052348015600f57600080fd5b5060043610603c5760003560e01c80633fb5c1cb1460415780638381f58a146053578063d09de08a14606d575b600080fd5b6051604c3660046083565b600055565b005b605b60005481565b60405190815260200160405180910390f35b6051600080549080607c83609b565b9190505550565b600060208284031215609457600080fd5b5035919050565b60006001820160ba57634e487b7160e01b600052601160045260246000fd5b506001019056fea2646970667358221220f0cfb2159c518c3da0ad864362bad5dc0715514a9ab679237253d506773a0a1b64736f6c63430008130033";
    pub const COUNTER_WITH_STORAGE_MAPPING: &str = r#"608060405234801561001057600080fd5b5060056000806001815260200190815260200160002081905550610197806100396000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063846719e01461003b578063d78233d61461006b575b600080fd5b6100556004803603810190610050919061010a565b61009b565b6040516100629190610146565b60405180910390f35b6100856004803603810190610080919061010a565b6100b7565b6040516100929190610146565b60405180910390f35b6000806000838152602001908152602001600020549050919050565b60006020528060005260406000206000915090505481565b600080fd5b6000819050919050565b6100e7816100d4565b81146100f257600080fd5b50565b600081359050610104816100de565b92915050565b6000602082840312156101205761011f6100cf565b5b600061012e848285016100f5565b91505092915050565b610140816100d4565b82525050565b600060208201905061015b6000830184610137565b9291505056fea2646970667358fe122066b287fef10118cba238fe38953bfefe938afefefefefe94fefe3682fefefefe64736f6c63430008110033"#;
}
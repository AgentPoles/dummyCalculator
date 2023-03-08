#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]

mod  ink{
/* Ink! is an embedded domain-specific language (eDSL) 
based on Rust, developed by Parity, and utilized to 
write Web Assembly (Wasm) smart contracts for 
Substrate-based blockchains that include the Contracts pallet. 
The Contracts pallet is a module that can be added to any blockchain
built with Substrate to enable smart contract functionalities.*/
}



struct Erc20Token {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
}
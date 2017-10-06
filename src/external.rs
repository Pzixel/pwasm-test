use parity_hash::{H256, Address};
use bigint::U256;

pub struct Error;

pub trait External {
    fn storage_read(&mut self, _key: &H256) -> Result<[u8; 32], Error>  {
        unimplemented!();
    }
    fn storage_write(&mut self, _key: &H256, _value: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }
    fn suicide(&mut self, _refund: &Address) {
        unimplemented!();
    }
    fn create(&mut self, _endowment: U256, _code: &[u8]) -> Result<Address, Error> {
        unimplemented!();
    }
    fn call(&mut self, _address: &Address, _val: U256, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
        unimplemented!();
    }
    fn call_code(&mut self, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
        unimplemented!();
    }
    fn static_call(&mut self, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
        unimplemented!();
    }
    fn debug_log(&mut self, _msg: String) {
        unimplemented!();
    }
    fn blockhash(&mut self, _number: u64) -> Result<H256, Error> {
        unimplemented!();
    }
    fn coinbase(&mut self) -> Address {
        unimplemented!();
    }
    fn timestamp(&mut self) -> u64 {
        unimplemented!();
    }
    fn blocknumber(&mut self) -> u64 {
        unimplemented!();
    }
    fn difficulty(&mut self) -> U256 {
        unimplemented!();
    }
    fn gas_limit(&mut self) -> U256 {
        unimplemented!();
    }
    fn sender(&mut self) -> Address {
        unimplemented!();
    }
    fn origin(&mut self) -> Address {
        unimplemented!();
    }
    fn value(&mut self) -> U256 {
        unimplemented!();
    }
    fn address(&mut self) -> Address {
        unimplemented!();
    }
    fn balance(&mut self, _address: &Address) -> U256 {
        unimplemented!();
    }
}

pub struct ExternalImpl {}

impl ExternalImpl {
    pub fn new() -> ExternalImpl {
        ExternalImpl{}
    }
}

impl External for ExternalImpl {
}

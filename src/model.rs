use std::str::FromStr;

use ethers::types::U256;

use eyre::Result;
use jsonp::Pointer;

/// A transaction.
#[derive(Debug)]
pub struct Transaction(pub String);

/// A transaction receipt.
#[derive(Debug)]
pub struct TransactionReceipt(pub String);

impl TransactionReceipt {
    /// Returns the transaction actual fee.
    /// # Returns
    /// The transaction actual fee.
    pub fn actual_fee(&self) -> Result<U256> {
        let p = Pointer::default();
        let raw = self.0.as_str();
        let actual_fee: &str = p.dotted(raw, ".actual_fee")?;
        Ok(U256::from_str(actual_fee)?)
    }
}

#[derive(Debug)]
pub struct Block(pub String);

impl Block {
    /// Returns the block gas price.
    /// # Returns
    /// The block gas price.
    pub fn gas_price(&self) -> Result<U256> {
        let p = Pointer::default();
        let raw = self.0.as_str();
        let gas_used: &str = p.dotted(raw, ".gas_price")?;
        Ok(U256::from_str(gas_used)?)
    }
}

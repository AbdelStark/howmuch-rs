use std::str::FromStr;

use ethers::types::U256;

use crate::resources::CairoResources;
use eyre::Result;
use jsonp::Pointer;

/// A transaction.
#[derive(Debug)]
pub struct Transaction(pub String);

/// A transaction receipt.
#[derive(Debug)]
pub struct TransactionReceipt(pub String);

impl TransactionReceipt {
    /// Attempts to create a TransactionReceipt from a file.
    pub fn try_from_file(filename: &str) -> Result<Self> {
        let s = std::fs::read_to_string(filename)?;
        Ok(Self(s))
    }

    /// Returns the transaction actual fee.
    /// # Returns
    /// The transaction actual fee.
    pub fn actual_fee(&self) -> Result<U256> {
        let p = Pointer::default();
        let raw = self.0.as_str();
        let actual_fee: &str = p.dotted(raw, ".actual_fee")?;
        Ok(U256::from_str(actual_fee)?)
    }

    /// Returns the resources used in this transaction.
    pub fn resources_used(&self) -> Result<CairoResources> {
        let j = json::parse(&self.0).unwrap();
        let exec_resources = &j["execution_resources"];
        let instance_counter = &exec_resources["builtin_instance_counter"];

        let category = "calls";
        let steps = exec_resources["n_steps"].as_f32().unwrap_or(0.0);
        let range_check = instance_counter["range_check_builtin"]
            .as_f32()
            .unwrap_or(0.0);
        let pedersen = instance_counter["pedersen_builtin"].as_f32().unwrap_or(0.0);
        let bitwise = instance_counter["bitwise_builtin"].as_f32().unwrap_or(0.0);
        let ecdsa = instance_counter["ecdsa_builtin"].as_f32().unwrap_or(0.0);
        let ec_op = instance_counter["ec_op_builtin"].as_f32().unwrap_or(0.0);
        Ok(CairoResources::new(
            category,
            steps,
            pedersen,
            range_check,
            ecdsa,
            bitwise,
            ec_op,
        ))
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

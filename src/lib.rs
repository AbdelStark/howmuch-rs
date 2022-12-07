//! # Howmuch
//! A library providing helpers for various StarkNet fees related tasks.
//! It can be used as a library or as a CLI.
//!
//! ## Library
//!
//! ### Fees
//!
//! #### Estimate on network
//!
//! Estimate the fees of a transaction on another network.
//!
//! ```rust
//! use howmuch_rs::{estimate_cost_on_network, model::Transaction};
//!
//! let tx_hash = "0x073251e7ff3843c4954aa2e7f38d8c29034e34a1ddbaeb1e62605ec10ca22367";
//! let source_network_gateway_url = "https://alpha4-2.starknet.io/feeder_gateway";
//! let destination_network_gateway_url = "https://alpha-mainnet.starknet.io/feeder_gateway";
//! let source_block_number = Some(21410);
//! let destination_block_number = Some(15925);
//! let fees = estimate_cost_on_network(
//!   tx_hash,
//!   &source_network_gateway_url,
//!   &destination_network_gateway_url,
//!   &source_block_number,
//!   &destination_block_number
//! ).unwrap();
//! println!("{}", fees);
//! ```
//!
//! ## CLI
//!
//! ### Fees
//!
//! #### Estimate on network
//!
//! Estimate the fees of a transaction on another network.
//!
//! ```bash
//! $ howmuch fees estimate-on-network \
//! --tx-hash 0x073251e7ff3843c4954aa2e7f38d8c29034e34a1ddbaeb1e62605ec10ca22367 \
//! --source-network-gateway-url https://alpha4-2.starknet.io/feeder_gateway \
//! --destination-network-gateway-url https://alpha-mainnet.starknet.io/feeder_gateway \
//! --source-block-number 21410 \
//! --destination-block-number 15925
//! ```
pub mod cli;
pub mod model;
use ethers::{types::U256, utils};
use eyre::Result;
use log::debug;
use model::{Block, Transaction, TransactionReceipt};

/// Simulate cost of a transaction on another network.
/// # Arguments
/// * `tx_hash` - The transaction hash.
/// * `source_network_gateway_url` - The source network gateway URL.
/// * `destination_network_gateway_url` - The destination network gateway URL.
/// * `source_block_number` - The source block number.
/// * `destination_block_number` - The destination block number.
/// # Returns
/// The estimated fees.
/// # Example
/// ```rust
/// use howmuch_rs::{estimate_cost_on_network, model::Transaction};
/// let tx_hash = "0x073251e7ff3843c4954aa2e7f38d8c29034e34a1ddbaeb1e62605ec10ca22367";
/// let source_network_gateway_url = "https://alpha4-2.starknet.io/feeder_gateway";
/// let destination_network_gateway_url = "https://alpha-mainnet.starknet.io/feeder_gateway";
/// let source_block_number = Some(21410);
/// let destination_block_number = Some(15925);
/// let fees = estimate_cost_on_network(
///   tx_hash,
///   &source_network_gateway_url,
///   &destination_network_gateway_url,
///   &source_block_number,
///   &destination_block_number
/// ).unwrap();
/// println!("{}", fees);
/// ```
///
///
pub fn estimate_cost_on_network(
    tx_hash: &str,
    source_network_gateway_url: &str,
    destination_network_gateway_url: &str,
    source_block_number: &Option<u32>,
    destination_block_number: &Option<u32>,
) -> Result<String> {
    let source_block_number = match source_block_number {
        Some(block_number) => block_number.to_string(),
        None => "latest".to_string(),
    };
    let destination_block_number = match destination_block_number {
        Some(block_number) => block_number.to_string(),
        None => "latest".to_string(),
    };
    debug!("querying transaction {} on source network", tx_hash);
    let source_tx = query_tx_receipt(tx_hash, source_network_gateway_url)?;
    let actual_fee = source_tx.actual_fee()?;
    debug!("transaction actual fee: {}", actual_fee);
    debug!("querying block {} on source network", source_block_number);
    let source_block = query_block(&source_block_number, source_network_gateway_url)?;
    let gas_price = source_block.gas_price()?;
    debug!("source block gas price: {}", gas_price);
    let tx_static_fee = compute_static_tx_fee(actual_fee, gas_price)?;
    debug!("transaction static fee: {}", tx_static_fee);
    debug!(
        "querying block {} on destination network",
        destination_block_number
    );
    let destination_block =
        query_block(&destination_block_number, destination_network_gateway_url)?;
    let destination_gas_price = destination_block.gas_price()?;
    debug!("destination block gas price: {}", destination_gas_price);
    let destination_tx_actual_fee = compute_actual_tx_fee(tx_static_fee, destination_gas_price)?;
    debug!(
        "transaction actual fee on destination network: {}",
        destination_tx_actual_fee
    );
    let destination_tx_actual_fee_in_eth = utils::format_units(destination_tx_actual_fee, "ether")?;

    Ok(destination_tx_actual_fee_in_eth)
}

/// Query a transaction from a network.
/// # Arguments
/// * `tx_hash` - The transaction hash.
/// * `network_gateway_url` - The network gateway URL.
/// # Returns
/// The transaction.
/// # Example
/// ```
/// use howmuch_rs::query_tx;
/// let tx = query_tx("0x073251e7ff3843c4954aa2e7f38d8c29034e34a1ddbaeb1e62605ec10ca22367", "https://alpha4-2.starknet.io/feeder_gateway").unwrap();
/// ```
/// # Errors
/// If the transaction is not found, an error is returned.
pub fn query_tx(tx_hash: &str, network_gateway_url: &str) -> Result<Transaction> {
    Ok(Transaction(http_get(&format!(
        "{}/get_transaction?transactionHash={}",
        network_gateway_url, tx_hash
    ))?))
}

/// Query a transaction from a network.
/// # Arguments
/// * `tx_hash` - The transaction hash.
/// * `network_gateway_url` - The network gateway URL.
/// # Returns
/// The transaction receipt.
/// # Example
/// ```
/// use howmuch_rs::query_tx_receipt;
/// let tx_receipt = query_tx_receipt("0x073251e7ff3843c4954aa2e7f38d8c29034e34a1ddbaeb1e62605ec10ca22367", "https://alpha4-2.starknet.io/feeder_gateway").unwrap();
/// ```
/// # Errors
/// If the transaction is not found, an error is returned.
pub fn query_tx_receipt(tx_hash: &str, network_gateway_url: &str) -> Result<TransactionReceipt> {
    Ok(TransactionReceipt(http_get(&format!(
        "{}/get_transaction_receipt?transactionHash={}",
        network_gateway_url, tx_hash
    ))?))
}

/// Query a block from a network.
/// # Arguments
/// * `block_number` - The block number.
/// * `network_gateway_url` - The network gateway URL.
/// # Returns
/// The block.
/// # Example
/// ```
/// use howmuch_rs::query_block;
/// let block = query_block("latest", "https://alpha4-2.starknet.io/feeder_gateway").unwrap();
/// ```
pub fn query_block(block_number: &str, network_gateway_url: &str) -> Result<Block> {
    Ok(Block(http_get(&format!(
        "{}/get_block?blockNumber={}",
        network_gateway_url, block_number
    ))?))
}

/// Compute the static part of the transaction fee.
/// # Arguments
/// * `actual_fee` - The actual fee of the transaction.
/// * `block_gas_price` - The block gas price.
/// # Returns
/// The static part of the transaction fee.
/// # Example
/// ```
/// use howmuch_rs::compute_static_tx_fee;
/// use ethers::types::U256;
/// let static_fee = compute_static_tx_fee(U256::from(100), U256::from(10)).unwrap();
/// ```
pub fn compute_static_tx_fee(actual_fee: U256, block_gas_price: U256) -> Result<U256> {
    if block_gas_price > actual_fee {
        return Err(eyre::eyre!("Block gas price must be lower than actual fee"));
    }
    Ok(actual_fee / block_gas_price)
}

/// Compute the actuall fee of a transaction.
/// # Arguments
/// * `tx_static_fee` - The static part of the transaction fee.
/// * `block_gas_price` - The block gas price.
/// # Returns
/// The actual fee of the transaction.
/// # Example
/// ```
/// use howmuch_rs::compute_actual_tx_fee;
/// use ethers::types::U256;
/// let actual_fee = compute_actual_tx_fee(U256::from(10), U256::from(10)).unwrap();
/// ```
pub fn compute_actual_tx_fee(tx_static_fee: U256, block_gas_price: U256) -> Result<U256> {
    Ok(tx_static_fee * block_gas_price)
}

/// Raw http GET request.
pub fn http_get(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)?;
    response.text().map_err(|e| e.into())
}

use clap::{Parser, Subcommand};

pub const DEFAULT_SOURCE_NETWORK_GATEWAY_URL: &str = "https://alpha4-2.starknet.io/feeder_gateway";
pub const DEFAULT_DESTINATION_NETWORK_GATEWAY_URL: &str =
    "https://alpha-mainnet.starknet.io/feeder_gateway";

/// How much ?
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// List of supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Ethereum related subcommands
    #[command(about = "Fees related subcommands")]
    Fees(FeesCommands),
}

/// Fees related commands.
#[derive(Parser, Debug)]
pub struct FeesCommands {
    /// Ethereum related subcommands.
    #[command(subcommand)]
    pub command: FeesSubCommands,
}

/// Fees related subcommands.
#[derive(Subcommand, Debug)]
pub enum FeesSubCommands {
    /// Estimate fee from a network to another.
    EstimateOnNetwork {
        /// The transaction hash on the source network.
        #[arg(short, long, value_name = "TX_HASH")]
        tx_hash: String,
        /// The source network gateway URL.
        /// If not provided, the default is goerli 2 testnet.
        #[arg(long, value_name = "SOURCE_NETWORK_GATEWAY_URL", default_value = DEFAULT_SOURCE_NETWORK_GATEWAY_URL)]
        source_network_gateway_url: String,
        /// The destination network gateway URL.
        /// If not provided, the default is mainnet.
        /// If the same as the source network, it will be ignored.
        #[arg(long, value_name = "DESTINATION_NETWORK_GATEWAY_URL", default_value = DEFAULT_DESTINATION_NETWORK_GATEWAY_URL)]
        destination_network_gateway_url: String,
        /// The source block number.
        /// If not provided, the default is the latest block.
        #[arg(long, value_name = "SOURCE_BLOCK_NUMBER")]
        source_block_number: Option<u32>,
        /// The destination block number.
        /// If not provided, the default is the latest block.
        #[arg(long, value_name = "DESTINATION_BLOCK_NUMBER")]
        destination_block_number: Option<u32>,
    },
}

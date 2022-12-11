use clap::{Parser, Subcommand};

pub const DEFAULT_SOURCE_NETWORK_GATEWAY_URL: &str = "https://alpha4-2.starknet.io/feeder_gateway";
pub const DEFAULT_DESTINATION_NETWORK_GATEWAY_URL: &str =
    "https://alpha-mainnet.starknet.io/feeder_gateway";

pub const DEFAULT_STEPS_WEIGHT: &str = "0.05";
pub const DEFAULT_PEDERSEN_WEIGHT: &str = "1.6";
pub const DEFAULT_RANGE_CHECK_WEIGHT: &str = "0.8";
pub const DEFAULT_ECDSA_WEIGHT: &str = "102.4";
pub const DEFAULT_BITWISE_WEIGHT: &str = "3.2";
pub const DEFAULT_EC_OP_WEIGHT: &str = "51.2";

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
    /// Output a recap of used resources
    // TODO: Ideally find a way to have either `(tx_hash && source_network_gateway_url) || transaction_file` as mandatory args
    Summary {
        #[arg(short, long, value_name = "TX_HASH")]
        tx_hash: Option<String>,

        #[arg(long, default_value = DEFAULT_SOURCE_NETWORK_GATEWAY_URL)]
        gateway_url: Option<String>,

        #[arg(
            long,
            help = "File containing a transaction receipt. Overrides `tx_hash` and `source_network_gateway_url`"
        )]
        transaction_file: Option<String>,

        #[arg(long, help="Overrides the default steps weight", default_value = DEFAULT_STEPS_WEIGHT)]
        steps_weight: f32,

        #[arg(long, help="Overrides the default pedersen weight", default_value = DEFAULT_PEDERSEN_WEIGHT)]
        pedersen_weight: f32,

        #[arg(long, help="Overrides the default range_check weight", default_value = DEFAULT_RANGE_CHECK_WEIGHT)]
        range_check_weight: f32,

        #[arg(long, help="Overrides the default ecdsa weight", default_value = DEFAULT_ECDSA_WEIGHT)]
        ecdsa_weight: f32,

        #[arg(long, help="Overrides the default bitwise weight", default_value = DEFAULT_BITWISE_WEIGHT)]
        bitwise_weight: f32,

        #[arg(long, help="Overrides the default ec_op weight", default_value = DEFAULT_EC_OP_WEIGHT)]
        ec_op_weight: f32,

        #[arg(long, help = "Overrides the step count from the transaction receipt")]
        steps: Option<String>,

        #[arg(
            long,
            help = "Overrides the pedersen count from the transaction receipt"
        )]
        pedersen: Option<String>,

        #[arg(
            long,
            help = "Overrides the range_check count from the transaction receipt"
        )]
        range_check: Option<String>,

        #[arg(long, help = "Overrides the ecdsa count from the transaction receipt")]
        ecdsa: Option<String>,

        #[arg(
            long,
            help = "Overrides the bitwise count from the transaction receipt"
        )]
        bitwise: Option<String>,

        #[arg(long, help = "Overrides the ec_op count from the transaction receipt")]
        ec_op: Option<String>,
    },
}

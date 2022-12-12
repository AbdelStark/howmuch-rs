use clap::Parser;
use eyre::Result;
use howmuch_rs::{
    cli::{Cli, Commands, FeesSubCommands},
    estimate_cost_on_network,
};

fn main() -> Result<()> {
    env_logger::init();
    // Parse the CLI arguments.
    let cli = Cli::parse();
    // Dispatch the CLI command.
    match &cli.command {
        Commands::Fees(fees_commands) => match &fees_commands.command {
            FeesSubCommands::EstimateOnNetwork {
                tx_hash,
                source_network_gateway_url,
                destination_network_gateway_url,
                source_block_number,
                destination_block_number,
            } => {
                let actual_fees_on_destination_network = estimate_cost_on_network(
                    tx_hash,
                    source_network_gateway_url,
                    destination_network_gateway_url,
                    source_block_number,
                    destination_block_number,
                )?;
                println!("{} ETH", actual_fees_on_destination_network);
            }
        },
    }

    Ok(())
}

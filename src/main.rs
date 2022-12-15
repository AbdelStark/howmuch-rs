use clap::Parser;
use eyre::Result;
use howmuch_rs::{
    cli::{Cli, Commands, FeesSubCommands},
    estimate_cost_on_network,
    resources::{get_resources_used, Weights},
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
                usd,
            } => {
                let actual_fees_on_destination_network = estimate_cost_on_network(
                    tx_hash,
                    source_network_gateway_url,
                    destination_network_gateway_url,
                    source_block_number,
                    destination_block_number,
                    usd,
                )?;
                println!("{}", actual_fees_on_destination_network);
            }
            FeesSubCommands::Summary {
                tx_hash,
                gateway_url,
                transaction_file,
                steps_weight,
                pedersen_weight,
                range_check_weight,
                ecdsa_weight,
                bitwise_weight,
                ec_op_weight,
                steps,
                pedersen,
                range_check,
                ecdsa,
                bitwise,
                ec_op,
            } => {
                let weights = Weights::new(
                    "weight",
                    *steps_weight,
                    *pedersen_weight,
                    *range_check_weight,
                    *ecdsa_weight,
                    *bitwise_weight,
                    *ec_op_weight,
                );

                let mut resources_used = get_resources_used(
                    tx_hash.as_ref().map(|x| x.as_ref()),
                    gateway_url.as_ref().map(|x| x.as_ref()),
                    transaction_file.as_ref().map(|x| x.as_ref()),
                )?;
                resources_used.update(steps, pedersen, range_check, ecdsa, bitwise, ec_op);

                let table = resources_used.to_table(&weights);
                println!("{}", table);
            }
        },
    }

    Ok(())
}

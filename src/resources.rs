use crate::model::TransactionReceipt;
use crate::query_tx_receipt;
use eyre::Result;
use tabled::Tabled;
use tabled::{Alignment, Panel, Style, Table};

pub type Weights = CairoResources;

/// List of all the different resources a transaction can use
#[derive(Tabled, Copy, Clone)]
pub struct CairoResources {
    pub category: &'static str,
    pub steps: f32,
    pub pedersen: f32,
    pub range_check: f32,
    pub ecdsa: f32,
    pub bitwise: f32,
    pub ec_op: f32,
}

impl CairoResources {
    pub fn new(
        category: &'static str,
        steps: f32,
        pedersen: f32,
        range_check: f32,
        ecdsa: f32,
        bitwise: f32,
        ec_op: f32,
    ) -> Self {
        Self {
            category,
            steps,
            pedersen,
            range_check,
            ecdsa,
            bitwise,
            ec_op,
        }
    }

    fn extract_fee(&self, weights: &Self) -> Self {
        Self {
            category: "fee",
            steps: self.steps * weights.steps,
            pedersen: self.pedersen * weights.pedersen,
            range_check: self.range_check * weights.range_check,
            ecdsa: self.ecdsa * weights.ecdsa,
            bitwise: self.bitwise * weights.bitwise,
            ec_op: self.ec_op * weights.ec_op,
        }
    }

    fn get_limiting_factor(&self) -> f32 {
        *[
            self.steps,
            self.pedersen,
            self.range_check,
            self.ecdsa,
            self.bitwise,
            self.ec_op,
        ]
        .iter()
        .max_by(|&&x, &y| x.total_cmp(y))
        .unwrap_or(&0.0)
    }

    pub fn update(
        &mut self,
        steps: &Option<String>,
        pedersen: &Option<String>,
        range_check: &Option<String>,
        ecdsa: &Option<String>,
        bitwise: &Option<String>,
        ec_op: &Option<String>,
    ) {
        self.steps = steps
            .as_ref()
            .map(|x| x.parse().expect("Invalid steps number"))
            .unwrap_or(self.steps);
        self.pedersen = pedersen
            .as_ref()
            .map(|x| x.parse().expect("Invalid pedersen number"))
            .unwrap_or(self.pedersen);
        self.range_check = range_check
            .as_ref()
            .map(|x| x.parse().expect("Invalid range_check number"))
            .unwrap_or(self.range_check);
        self.ecdsa = ecdsa
            .as_ref()
            .map(|x| x.parse().expect("Invalid ecdsa number"))
            .unwrap_or(self.ecdsa);
        self.bitwise = bitwise
            .as_ref()
            .map(|x| x.parse().expect("Invalid bitwise number"))
            .unwrap_or(self.bitwise);
        self.ec_op = ec_op
            .as_ref()
            .map(|x| x.parse().expect("Invalid ec_op number"))
            .unwrap_or(self.ec_op);
    }

    pub fn to_table(&self, weights: &Weights) -> String {
        let fee = self.extract_fee(weights);

        let mut table = Table::new(vec![*self, *weights, fee]);
        table
            .with(Panel::footer(format!(
                "Limiting factor: {}",
                fee.get_limiting_factor()
            )))
            .with(Alignment::center())
            .with(Style::modern());
        table.to_string()
    }
}

pub fn get_resources_used(
    tx_hash: Option<&str>,
    source_network_gateway_url: Option<&str>,
    transaction_file: Option<&str>,
) -> Result<CairoResources> {
    let tx_receipt = match (tx_hash, source_network_gateway_url, transaction_file) {
        (_, _, Some(filename)) => TransactionReceipt::try_from_file(filename)?,
        (Some(hash), Some(url), None) => query_tx_receipt(hash, url)?,
        (_, _, _) => {
            return Err(eyre::eyre!(
                "Provide either a filename or a transaction_hash and source network gateway url"
            ));
        }
    };

    tx_receipt.resources_used()
}

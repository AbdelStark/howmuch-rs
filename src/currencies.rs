use ethers::types::U256;

const URL: &'static str =
    "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";

fn get_eth_price() -> Option<f32> {
    let body = reqwest::blocking::get(URL).ok()?.text().ok()?;

    let parsed = json::parse(&body).ok()?;
    let num = parsed["ethereum"]["usd"].as_f32()?;

    Some(num)
}

/// Returns the string corresponding to the dollar cost in dollars (USD).
/// The oracle is coingecko (see [`URL`]).
/// Output is formatted with a precision of 4
pub fn format_dollar_cost(fee: U256) -> String {
    let eth_price = get_eth_price();
    let eth_price = match eth_price {
        None => return String::from("could not display USD estimate: failed to get eth price"),
        Some(p) => p,
    };

    let fee_in_eth = ethers::utils::format_units(fee, "ether");
    let fee_in_eth = match fee_in_eth {
        Err(_) => return String::from("could not display USD estimate: failed to convert fees"),
        Ok(fee) => fee,
    };

    let float = fee_in_eth.parse::<f32>();
    let float = match float {
        Err(_) => {
            return String::from("could not display USD estimate: failed to parse the number")
        }
        Ok(f) => f,
    };

    let dollar_fee = float * eth_price;
    format!("${:.4} USD", dollar_fee)
}

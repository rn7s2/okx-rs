use okx_rs::api::v5::GetHistoryCandles;
use okx_rs::api::{blocking, Options, Production};

fn main() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1000;

    println!("now: {}", now);

    let client = blocking::Rest::new(Options::new(Production));
    let response = client
        .request(GetHistoryCandles {
            inst_id: "BTC-USDT".to_string(),
            after: Some(now),
            before: None,
            bar: None,
            limit: None,
        })
        .unwrap();
    println!("{}", serde_json::to_string(&response).unwrap());
}

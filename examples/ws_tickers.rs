use okx_rs::websocket::conn::Tickers;
use tungstenite::Message;

use okx_rs::api::v5::ws_convert::TryParseEvent;
use okx_rs::websocket::WebsocketChannel;

fn main() {
    let (mut client, response) =
        tungstenite::connect("wss://ws.okx.com:8443/ws/v5/public").unwrap();
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    println!("{:?}", response.headers());

    client
        .send(Tickers("BTC-USDT".into()).subscribe_message().into())
        .unwrap();

    loop {
        let msg = match client.read() {
            Ok(Message::Text(msg)) => msg,
            Err(err) => {
                panic!("{:?}", err);
            }
            _ => continue,
        };

        if let Ok(Some(tickers)) = Tickers::try_parse(&msg) {
            println!("{:?}", tickers.data);
        }
    }
}

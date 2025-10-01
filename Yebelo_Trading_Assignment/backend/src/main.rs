mod kafka;
mod rsi;

use kafka::{create_consumer, create_producer, consume_messages, publish_rsi};
use rsi::calculate_rsi;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Starting Rust RSI backend...");

    // Kafka setup
    let brokers = "localhost:9092";
    let trade_topic = "trade-data";
    let rsi_topic = "rsi-data";
    let group_id = "rsi-consumer-group";

    let consumer = create_consumer(brokers, group_id, trade_topic).await;
    let producer = create_producer(brokers).await;

    // Store prices per token
    let mut price_history: HashMap<String, Vec<f64>> = HashMap::new();

    consume_messages(consumer, move |msg| {
        if let Ok(payload) = msg.payload_view::<str>() {
            if let Some(json_str) = payload {
                if let Ok(trade) = serde_json::from_str::<Value>(json_str) {
                    let token = trade["token_address"].as_str().unwrap_or("").to_string();
                    let price = trade["price_in_sol"].as_f64().unwrap_or(0.0);

                    let entry = price_history.entry(token.clone()).or_insert(Vec::new());
                    entry.push(price);

                    // Calculate RSI if enough data (14-period)
                    if entry.len() >= 14 {
                        let rsi = calculate_rsi(entry.clone(), 14);
                        // Publish RSI to rsi-data topic
                        let rsi_msg = serde_json::json!({
                            "token_address": token,
                            "rsi": rsi
                        });
                        let _ = publish_rsi(&producer, rsi_topic, rsi_msg.to_string());
                    }
                }
            }
        }
    }).await;
}

use std::sync::Arc;

// ============================================================================
// src/main.rs
// ============================================================================
use log::LevelFilter;
use tokio::sync::mpsc;
use rumqttc::{Event, Packet};
mod config;
mod database;
mod mqtt_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    let config = crate::config::load_config("ssn_conf.yaml")?;
    log::info!("Starting SSN IoT System: {}", config.app.name);
    log::info!("Account: {}", config.ssn.account);

    // Initialize database client
    let db_client = if let Some(url) = &config.app.postgrest_url {
        Some(Arc::new(crate::database::DatabaseClient::new(url.clone())))
    } else {
        None
    };

    // Initialize MQTT client
    let (mqtt_client, mut eventloop) = crate::mqtt_client::SsnMqttClient::new(
        config.ssn.account,
        &config.app.mqtt_host,
        config.app.mqtt_port,
        &format!("{}persist", config.app.mqtt_broker_client_id),
        &config.app.mqtt_broker_user,
        &config.app.mqtt_broker_pass,
    )
    .await?;

    let mqtt_client = Arc::new(mqtt_client);

    // Subscribe to topics
    mqtt_client.subscribe_topics().await?;

    log::info!("System started successfully");

    // Main event loop
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let topic = &p.topic;
                let payload = String::from_utf8_lossy(&p.payload);
                
                log::debug!("Received: {} -> {}", topic, payload);

                // Parse topic and handle message
                if let Some((account, obj, device, channel)) = parse_topic(&topic) {
                    log::info!("handle message from topic {}", topic);
                    if account == config.ssn.account {
                        if let Ok(value) = payload.parse::<f64>() {
                            let ts = chrono::Utc::now().timestamp();
                            
                            // Store to database
                            if let Some(ref db) = db_client {
                                if let Err(e) = db.set_device_value(
                                    account, obj, &device, channel, value, 0, Some(ts)
                                ).await {
                                    log::error!("Database error: {}", e);
                                }
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                log::error!("MQTT error: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}

fn parse_topic(topic: &str) -> Option<(u32, u32, String, u32)> {
    let parts: Vec<&str> = topic.split('/').collect();
    if parts.len() >= 8 && parts[1] == "ssn" && parts[2] == "acc" {
        let account = parts[3].parse().ok()?;
        let obj = parts[5].parse().ok()?;
        let device = parts[7].to_string();
        let channel = parts[8].parse().ok()?;
        Some((account, obj, device, channel))
    } else {
        None
    }
}

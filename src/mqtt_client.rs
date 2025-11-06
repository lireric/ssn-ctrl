// ============================================================================
// src/mqtt_client.rs
// ============================================================================
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

pub struct SsnMqttClient {
    client: AsyncClient,
    account: u32,
    host: String,
    port: u16,
    client_id: String,
    username: String,
    password: String,
}

impl SsnMqttClient {
    pub async fn new(
        account: u32,
        host: &str,
        port: u16,
        client_id: &str,
        username: &str,
        password: &str,
    ) -> anyhow::Result<(Self, EventLoop)> {
        let mut mqtt_opts = MqttOptions::new(client_id, host, port);
        mqtt_opts.set_credentials(username, password);
        mqtt_opts.set_keep_alive(Duration::from_secs(60));
        // mqtt_opts.set_connection_timeout(10);

        let (client, eventloop) = AsyncClient::new(mqtt_opts, 10);

        Ok((
            Self {
                client,
                account,
                host: host.to_string(),
                port,
                client_id: client_id.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            },
            eventloop,
        ))
    }

    pub fn recreate_eventloop(&self) -> EventLoop {
        log::info!("recreate_eventloop at broker: {}", &self.host);
        let mut mqtt_opts = MqttOptions::new(&self.client_id, &self.host, self.port);
        mqtt_opts.set_credentials(&self.username, &self.password);
        mqtt_opts.set_keep_alive(Duration::from_secs(60));
        // mqtt_opts.set_connection_timeout(10);

        let (client, eventloop) = AsyncClient::new(mqtt_opts, 10);
        eventloop
    }

    pub async fn subscribe_topics(&self) -> anyhow::Result<()> {
        let topics = vec![
            format!("/ssn/acc/{}/obj/+/device/+/+/out", self.account),
            format!("/ssn/acc/{}/obj/+/commands", self.account),
        ];

        for topic in topics {
            self.client.subscribe(&topic, QoS::AtMostOnce).await?;
            log::info!("Subscribed to: {}", topic);
        }

        Ok(())
    }

    pub async fn publish_sensor_value(
        &self,
        obj: u32,
        device: &str,
        channel: u32,
        value: f64,
        timestamp: i64,
        action_id: u32,
    ) -> anyhow::Result<()> {
        let topic = format!(
            "/ssn/acc/{}/obj/{}/device/{}/{}/out",
            self.account, obj, device, channel
        );

        // Publish simple value
        self.client
            .publish(&topic, QoS::AtMostOnce, false, value.to_string())
            .await?;

        // Publish JSON with full data
        let json_data = serde_json::json!({
            "a": action_id,
            "d": device,
            "c": channel,
            "v": value,
            "t": timestamp,
            "pub_ts": chrono::Utc::now().timestamp()
        });

        self.client
            .publish(
                &format!("{}_json", topic),
                QoS::AtMostOnce,
                false,
                json_data.to_string(),
            )
            .await?;

        // Publish event if triggered by action
        if action_id > 0 {
            let event_topic = format!("/ssn/acc/{}/obj/{}/event", self.account, obj);
            self.client
                .publish(&event_topic, QoS::AtMostOnce, false, json_data.to_string())
                .await?;
        }

        Ok(())
    }
}
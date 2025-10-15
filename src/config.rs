// ============================================================================
// src/config.rs
// ============================================================================
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub ssn: SsnConfig,
    pub app: AppConfig,
    pub persist: Option<PersistConfig>,
    pub bot: Option<BotConfig>,
    pub sensors: Option<SensorsConfig>,
    pub actions: Option<Vec<ActionConfig>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SsnConfig {
    #[serde(rename = "ACCOUNT")]
    pub account: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub name: String,
    #[serde(rename = "MQTT_PORT")]
    pub mqtt_port: u16,
    #[serde(rename = "MQTT_HOST")]
    pub mqtt_host: String,
    #[serde(rename = "MQTT_BROKER_USER")]
    pub mqtt_broker_user: String,
    #[serde(rename = "MQTT_BROKER_PASS")]
    pub mqtt_broker_pass: String,
    #[serde(rename = "MQTT_BROKER_CLIENT_ID")]
    pub mqtt_broker_client_id: String,
    #[serde(rename = "POSTGRESTURL")]
    pub postgrest_url: Option<String>,
    #[serde(rename = "LOG_TO_MQTT")]
    pub log_to_mqtt: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersistConfig {
    pub start: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BotConfig {
    pub start: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorsConfig {
    pub obj: u32,
    pub gpio: Option<GpioConfig>,
    pub ds18b20: Option<Ds18b20Config>,
    pub watchdog_tcp: Option<WatchdogTcpConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GpioConfig {
    pub scan_rate: u32,
    pub pins: Vec<GpioPin>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GpioPin {
    pub id: String,
    pub gpiochip: u32,
    pub number: u32,
    #[serde(rename = "type")]
    pub pin_type: String,
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ds18b20Config {
    pub masters: Vec<Ds18b20Master>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ds18b20Master {
    pub scan_rate: u32,
    pub path: String,
    pub name: String,
    pub devices: Vec<Ds18b20Device>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ds18b20Device {
    pub id: String,
    pub name: String,
    pub resolution: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchdogTcpConfig {
    pub destinations: Vec<WatchdogDestination>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchdogDestination {
    pub id: String,
    pub address: String,
    pub scan_rate: u32,
    pub command: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionConfig {
    pub id: u32,
    pub expression: String,
    pub act: Vec<String>,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

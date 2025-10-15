// ============================================================================
// src/database.rs
// ============================================================================
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceInfo {
    pub account: u32,
    pub object: u32,
    pub device: String,
    pub channel: String,
    pub dev_name: String,
    pub dev_descr: Option<String>,
    pub dev_scale: Option<String>,
    pub dev_unit_id: Option<u32>,
    pub dev_grp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeleData {
    pub td_account: u32,
    pub td_object: u32,
    pub td_device: String,
    pub td_channel: u32,
    pub td_dev_ts: i64,
    pub td_store_ts: i64,
    pub td_dev_value: f64,
    pub td_action: u32,
}

pub struct DatabaseClient {
    base_url: String,
    client: reqwest::Client,
    device_cache: tokio::sync::Mutex<HashMap<String, DeviceInfo>>,
}

impl DatabaseClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
            device_cache: tokio::sync::Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_device_info(&self, account: u32, device: &str) -> anyhow::Result<Option<DeviceInfo>> {
        // Check cache first
        {
            let cache = self.device_cache.lock().await;
            if let Some(info) = cache.get(device) {
                return Ok(Some(info.clone()));
            }
        }

        // Fetch from database
        let url = format!(
            "{}/devices?account=eq.{}&device=eq.{}&limit=1",
            self.base_url, account, device
        );

        let response = self.client.get(&url).send().await?;
        let devices: Vec<DeviceInfo> = response.json().await?;

        if let Some(device_info) = devices.into_iter().next() {
            let mut cache = self.device_cache.lock().await;
            cache.insert(device.to_string(), device_info.clone());
            Ok(Some(device_info))
        } else {
            Ok(None)
        }
    }

    pub async fn get_device_value(
        &self,
        account: u32,
        object: u32,
        device: &str,
        channel: u32,
    ) -> anyhow::Result<Option<f64>> {
        let url = format!(
            "{}/ssn_teledata?td_account=eq.{}&td_object=eq.{}&td_device=eq.{}&order=td_store_ts.desc&limit=1",
            self.base_url, account, object, device
        );

        let response = self.client.get(&url).send().await?;
        let data: Vec<TeleData> = response.json().await?;

        Ok(data.into_iter().next().map(|d| d.td_dev_value))
    }

    pub async fn set_device_value(
        &self,
        account: u32,
        object: u32,
        device: &str,
        channel: u32,
        value: f64,
        action_id: u32,
        dev_ts: Option<i64>,
    ) -> anyhow::Result<()> {
        let ts = chrono::Utc::now().timestamp();
        let dev_ts = dev_ts.unwrap_or(ts);

        let data = vec![TeleData {
            td_account: account,
            td_object: object,
            td_device: device.to_string(),
            td_channel: channel,
            td_dev_ts: dev_ts,
            td_store_ts: ts,
            td_dev_value: value,
            td_action: action_id,
        }];

        let url = format!("{}ssn_teledata", self.base_url);
        log::info!("set device value. url={} data={:?}", url, data);
        self.client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
}

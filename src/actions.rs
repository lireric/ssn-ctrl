// ============================================================================
// src/actions.rs
// ============================================================================
use std::collections::HashMap;
use regex::Regex;

pub struct ActionEngine {
    actions: Vec<Action>,
    get_value_fn: Box<dyn Fn(&str, u32) -> Option<f64> + Send + Sync>,
    set_value_fn: Box<dyn Fn(&str, u32, f64, u32) + Send + Sync>,
}

#[derive(Debug, Clone)]
struct Action {
    id: u32,
    expression: String,
    devices: Vec<(String, u32)>,
    act_expressions: Vec<String>,
}

impl ActionEngine {
    pub fn new<F, G>(get_fn: F, set_fn: G) -> Self
    where
        F: Fn(&str, u32) -> Option<f64> + Send + Sync + 'static,
        G: Fn(&str, u32, f64, u32) + Send + Sync + 'static,
    {
        Self {
            actions: Vec::new(),
            get_value_fn: Box::new(get_fn),
            set_value_fn: Box::new(set_fn),
        }
    }

    pub fn add_action(&mut self, id: u32, expression: String, act_expressions: Vec<String>) {
        let devices = Self::parse_devices(&expression);
        log::info!("Added action {}: {} devices in expression", id, devices.len());
        
        self.actions.push(Action {
            id,
            expression,
            devices,
            act_expressions,
        });
    }

    fn parse_devices(expr: &str) -> Vec<(String, u32)> {
        let re = Regex::new(r#"d\("?(\w+)"?\s*,\s*(\d+)\)"#).unwrap();
        let mut devices = Vec::new();

        for cap in re.captures_iter(expr) {
            if let (Some(dev), Some(ch)) = (cap.get(1), cap.get(2)) {
                let device = dev.as_str().to_string();
                let channel = ch.as_str().parse().unwrap_or(0);
                devices.push((device, channel));
            }
        }

        devices
    }

    pub fn get_actions_for_device(&self, device: &str, channel: u32) -> Vec<&Action> {
        self.actions
            .iter()
            .filter(|action| {
                action.devices.iter().any(|(d, c)| d == device && *c == channel)
            })
            .collect()
    }

    pub fn apply_actions(&self, device: &str, channel: u32) {
        let actions = self.get_actions_for_device(device, channel);
        
        for action in actions {
            log::debug!("Evaluating action {}", action.id);
            // In a real implementation, you would evaluate the expression
            // This is a simplified placeholder
            // For production, consider using a proper expression parser/evaluator
        }
    }
}

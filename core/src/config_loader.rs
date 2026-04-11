use std::fs;
use serde_yaml;
use crate::heuristics::HeuristicParams;

pub fn load_heuristics_from_config(path: &str) -> HeuristicParams {
    let config_string = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return HeuristicParams::default(),
    };

    let config: serde_yaml::Value = match serde_yaml::from_str(&config_string) {
        Ok(v) => v,
        Err(_) => return HeuristicParams::default(),
    };

    config.get("heuristics")
        .and_then(|h| serde_yaml::from_value(h.clone()).ok())
        .unwrap_or_else(|| HeuristicParams::default())
}

pub fn load_available_memory_from_config(path: &str) -> usize {
    let config_string = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return 20480,
    };

    let config: serde_yaml::Value = match serde_yaml::from_str(&config_string) {
        Ok(v) => v,
        Err(_) => return 2048,
    };

    config.get("available_memory")
        .and_then(|v| v.as_i64())
        .map(|v| {
            if v < 128 {
                // If it's a small number, assume they meant GB and convert to MB
                (v * 1024) as usize
            } else {
                // Otherwise assume it's already in MB
                v as usize
            }
        })
        .unwrap_or(2048)
}

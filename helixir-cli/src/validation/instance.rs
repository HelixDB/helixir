use std::{fs, io, process::Command};

use serde_json::json;

pub fn load_instance_data() -> serde_json::Value {
    let instance_file = "helixdb-cfg/instance.json";

    if let Ok(content) = fs::read_to_string(instance_file) {
        serde_json::from_str(&content).unwrap_or_else(|_| create_default_instance_data())
    } else {
        create_default_instance_data()
    }
}

pub fn create_default_instance_data() -> serde_json::Value {
    json!({
        "instance_id": "",
        "created_entities": {
            "continents": [],
            "countries": [],
            "cities": []
        }
    })
}

pub fn save_instance_data(data: &serde_json::Value) -> Result<(), String> {
    let instance_file = "helixdb-cfg/instance.json";
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize instance data: {}", e))?;

    fs::write(instance_file, content)
        .map_err(|e| format!("Failed to write instance file: {}", e))?;

    Ok(())
}

pub fn save_created_entity(
    entity_type: &str,
    entity_data: &serde_json::Value,
) -> Result<(), String> {
    let mut instance_data = load_instance_data();
    if instance_data["created_entities"][entity_type].is_array() {
        instance_data["created_entities"][entity_type] = json!([entity_data]);
    } else {
        return Err(format!("Invalid entity type: {}", entity_type));
    }

    save_instance_data(&instance_data)
}

pub fn get_latest_entity_id(entity_type: &str) -> Option<String> {
    let instance_data = load_instance_data();

    if let Some(entities) = instance_data["created_entities"][entity_type].as_array() {
        if let Some(latest_entity) = entities.last() {
            return latest_entity["id"].as_str().map(|s| s.to_string());
        }
    }

    None
}

pub fn get_or_prompt_instance_id() -> Result<String, String> {
    let instance_data = load_instance_data();
    if let Some(instance_id) = instance_data["instance_id"].as_str() {
        if !instance_id.is_empty() {
            return Ok(instance_id.to_string());
        }
    }

    println!("First time running queries! We need your HelixDB instance ID.");
    println!("Run 'helix instances' in another terminal and copy your instance ID.");
    println!("Enter your instance ID:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    let instance_id = input.trim().to_string();

    let mut instance_data = load_instance_data();
    instance_data["instance_id"] = json!(instance_id.clone());
    save_instance_data(&instance_data)
        .map_err(|e| format!("Failed to save to instance.json: {}", e))?;

    println!("Instance ID saved! Future query runs will be automatic.");
    Ok(instance_id)
}

pub fn redeploy_instance(instance_id: &str) -> bool {
    println!("Running: helix redeploy {}", instance_id);
    let output = Command::new("helix")
        .arg("redeploy")
        .arg(instance_id)
        .output();

    match output {
        Ok(result) => {
            let stdout_str = String::from_utf8_lossy(&result.stdout);
            let stderr_str = String::from_utf8_lossy(&result.stderr);

            if stdout_str.contains("No Helix instance found")
                || stderr_str.contains("No Helix instance found")
            {
                println!("Error: Invalid instance ID '{}'", instance_id);
                println!("Run 'helix instances' to get your correct instance ID");
                return false;
            }

            if !stdout_str.is_empty() {
                println!("Output: {}", stdout_str);
            }
            if !stderr_str.is_empty() {
                println!("Error output: {}", stderr_str);
            }

            if stdout_str.contains("Parse error") || stderr_str.contains("Parse error") {
                println!("Deployment failed due to parse errors in queries.hx");
                return false;
            }

            if stdout_str.contains("Error compiling") || stderr_str.contains("Error compiling") {
                println!("Deployment failed due to compilation errors");
                return false;
            }

            if result.status.success() && !stdout_str.contains("No Helix instance found") {
                println!("Redeployed instance successfully");
                true
            } else {
                println!("Failed to redeploy instance");
                false
            }
        }
        Err(e) => {
            println!("Error running helix redeploy: {}", e);
            false
        }
    }
}

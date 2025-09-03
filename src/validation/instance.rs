use std::{fs, process::Command};

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
        "current_lesson": 0,
        "completed_lessons": [],
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

pub fn get_instance_id() -> Option<String> {
    let instance_data = load_instance_data();
    instance_data["instance_id"].as_str().map(|s| s.to_string())
}

pub fn save_instance_id(instance_id: &str) -> Result<(), String> {
    let mut instance_data = load_instance_data();
    instance_data["instance_id"] = json!(instance_id);
    save_instance_data(&instance_data)
}

pub fn save_created_entity(
    entity_type: &str,
    entity_data: &serde_json::Value,
) -> Result<(), String> {
    let mut instance_data = load_instance_data();
    if let Some(entities_array) = instance_data["created_entities"][entity_type].as_array_mut() {
        entities_array.push(entity_data.clone());
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

pub fn get_current_lesson() -> usize {
    let instance_data = load_instance_data();
    instance_data["current_lesson"].as_u64().unwrap_or(0) as usize
}

pub fn save_current_lesson(lesson_id: usize) -> Result<(), String> {
    let mut instance_data = load_instance_data();
    instance_data["current_lesson"] = json!(lesson_id);
    save_instance_data(&instance_data)
}

pub fn mark_lesson_completed(lesson_id: usize) -> Result<(), String> {
    let mut instance_data = load_instance_data();

    if let Some(completed_lessons) = instance_data["completed_lessons"].as_array_mut() {
        let lesson_value = json!(lesson_id);
        if !completed_lessons.contains(&lesson_value) {
            completed_lessons.push(lesson_value);
            completed_lessons.sort_by(|a, b| a.as_u64().cmp(&b.as_u64()));
        }
    }

    save_instance_data(&instance_data)
}

pub fn get_completed_lessons() -> Vec<usize> {
    let instance_data = load_instance_data();
    if let Some(completed_lessons) = instance_data["completed_lessons"].as_array() {
        completed_lessons
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as usize))
            .collect()
    } else {
        Vec::new()
    }
}

#[allow(dead_code)]
pub fn is_lesson_completed(lesson_id: usize) -> bool {
    get_completed_lessons().contains(&lesson_id)
}

pub fn redeploy_instance() -> bool {
    let instance_data = load_instance_data();
    let instance_id = instance_data["instance_id"].as_str().unwrap_or_default();

    let output = if instance_id.is_empty() {
        println!("Warning: No instance_id found in instance.json, falling back to helix compile");
        Command::new("helix").arg("compile").output()
    } else {
        println!("Deploying to cluster: {}", instance_id);
        Command::new("helix")
            .args(["stop", instance_id])
            .output()
            .unwrap();
        Command::new("helix")
            .args(["deploy", "-c", instance_id])
            .output()
    };

    match output {
        Ok(result) => {
            let stdout_str = String::from_utf8_lossy(&result.stdout);
            let stderr_str = String::from_utf8_lossy(&result.stderr);

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
            let has_success_indicators = stdout_str.contains("Successfully compiled")
                && stdout_str.contains("Successfully built")
                && stdout_str.contains("Successfully started");

            let has_basic_success = stdout_str.contains("Successfully transpiled")
                && stdout_str.contains("Helix instance found");
            let instance_running = stdout_str.contains("Helix instance found!")
                && stdout_str.contains("Available endpoints:");

            if result.status.success()
                || has_success_indicators
                || has_basic_success
                || instance_running
            {
                println!("Deployed successfully");
                true
            } else {
                println!("Failed to deploy");
                false
            }
        }
        Err(e) => {
            println!("Error running helix command: {}", e);
            false
        }
    }
}

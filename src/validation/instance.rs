use std::{fs, process::Command};

use serde_json::json;

pub fn load_instance_data() -> serde_json::Value {
    let instance_file = "instance.json";

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
        "created_entities": {
            "continents": [],
            "countries": [],
            "cities": []
        }
    })
}

pub fn save_instance_data(data: &serde_json::Value) -> Result<(), String> {
    let instance_file = "instance.json";
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
    println!("Building and deploying instance with 'helix build dev'...");

    // Run helix build dev
    let build_output = Command::new("helix")
        .args(["build", "dev"])
        .output();

    match build_output {
        Ok(result) => {
            let stdout_str = String::from_utf8_lossy(&result.stdout);
            let stderr_str = String::from_utf8_lossy(&result.stderr);

            if !stdout_str.is_empty() {
                println!("Build output: {}", stdout_str);
            }
            if !stderr_str.is_empty() {
                println!("Build errors: {}", stderr_str);
            }

            if stdout_str.contains("Parse error") || stderr_str.contains("Parse error") {
                println!("Build failed due to parse errors in queries.hx or schema.hx");
                return false;
            }

            if stdout_str.contains("Error compiling") || stderr_str.contains("Error compiling") {
                println!("Build failed due to compilation errors");
                return false;
            }

            if !result.status.success() {
                println!("Build failed with exit code: {:?}", result.status.code());
                return false;
            }

            println!("Build successful! Starting instance with 'helix push dev'...");
        }
        Err(e) => {
            println!("Error running helix build command: {}", e);
            return false;
        }
    }

    // Run helix push dev
    let push_output = Command::new("helix")
        .args(["push", "dev"])
        .output();

    match push_output {
        Ok(result) => {
            let stdout_str = String::from_utf8_lossy(&result.stdout);
            let stderr_str = String::from_utf8_lossy(&result.stderr);

            if !stdout_str.is_empty() {
                println!("Push output: {}", stdout_str);
            }
            if !stderr_str.is_empty() {
                println!("Push errors: {}", stderr_str);
            }

            if result.status.success() {
                println!("Instance deployed successfully!");
                true
            } else {
                println!("Failed to push instance");
                false
            }
        }
        Err(e) => {
            println!("Error running helix push command: {}", e);
            false
        }
    }
}

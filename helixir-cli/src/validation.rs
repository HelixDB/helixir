use crate::lesson_types::*;
use helix_db::{HelixDB, HelixDBClient};
use serde::de::{self, Expected};
use serde::{Deserialize, Serialize};
use std::ascii::escape_default;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::{fs, io, result};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    pub name: String,
    pub prop_type: String,
}

#[derive(Debug)]
pub struct EdgeInfo {
    pub from_type: String,
    pub to_type: String,
    pub properties: HashSet<Property>,
}

#[derive(Debug)]
pub struct ParsedSchema {
    pub nodes: HashMap<String, HashSet<Property>>,
    pub edges: HashMap<String, EdgeInfo>,
    pub vectors: HashMap<String, HashSet<Property>>,
}

pub struct PropertyErrors {
    pub missing: Vec<String>,
    pub extra: Vec<String>,
    pub wrong_type: Vec<(String, String, String)>,
}

#[derive(Debug)]
pub struct QueryValidator {
    client: HelixDB,
}

pub struct ValidationResult {
    pub is_correct: bool,
    pub missing_nodes: Vec<String>,
    pub extra_nodes: Vec<String>,
    pub property_errors: HashMap<String, PropertyErrors>,
    pub missing_edges: Vec<String>,
    pub extra_edges: Vec<String>,
    pub edge_errors: HashMap<String, EdgeErrors>,
    pub missing_vectors: Vec<String>,
    pub extra_vectors: Vec<String>,
    pub vector_errors: HashMap<String, PropertyErrors>,
}

pub struct EdgeErrors {
    pub from_type_mismatch: Option<(String, String)>,
    pub to_type_mismatch: Option<(String, String)>,
    pub property_errors: PropertyErrors,
}

impl QueryValidator {
    pub fn new() -> Self {
        return Self {
            client: HelixDB::new(None, None),
        };
    }

    pub async fn execute_and_compare(
        &self,
        query_name: &str,
        input: serde_json::Value,
        expected: serde_json::Value,
    ) -> anyhow::Result<(bool, String)> {
        match query_name {
            "createContinent" => {
                let input_de: AddContinentInput = serde_json::from_value(input)?;
                let db_result: CreateContinentResult = self
                    .client
                    .query("createContinent", &input_de)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e)
                    })?;

                let name_matches = db_result.continent.name == input_de.name;

                if name_matches {
                    let continent_data = json!({
                        "id": db_result.continent.id,
                        "name": db_result.continent.name
                    });
                    
                    if let Err(e) = save_created_entity("continents", &continent_data) {
                        println!("Warning: Could not save continent data: {}", e);
                    }

                    let success_msg = format!(
                        "Continent created successfully!\nDatabase result:\n{}\nSaved continent ID for future lessons.",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Query executed but result doesn't match expected.\nDatabase returned:\n{}\nExpected name: '{}'",
                        serde_json::to_string_pretty(&db_result)?,
                        input_de.name
                    );
                    Ok((false, error_msg))
                }
            }
            "createCountry" => {
                let continent_id = get_latest_entity_id("continents")
                    .ok_or_else(|| anyhow::anyhow!("No continent found. Please run lesson 5 first to create a continent."))?;
                
                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["continent_id"] = json!(continent_id);
                
                let input_de: CreateCountryInput = serde_json::from_value(input_obj)?;
                let db_result: CreateCountryResult = self
                    .client
                    .query("createCountry", &input_de)
                    .await?;

                let matches = db_result.country.name == input_de.name
                    && db_result.country.currency == input_de.currency
                    && db_result.country.population == input_de.population
                    && db_result.country.gdp == input_de.gdp;

                if matches {
                    let country_data = json!({
                        "id": db_result.country.id,
                        "name": db_result.country.name,
                        "currency": db_result.country.currency,
                        "population": db_result.country.population,
                        "gdp": db_result.country.gdp,
                        "continent_id": continent_id
                    });
                    
                    if let Err(e) = save_created_entity("countries", &country_data) {
                        println!("Warning: Could not save country data: {}", e);
                    }

                    let success_msg = format!(
                        "Country created successfully!\nDatabase result:\n{}\nSaved country ID for future lessons.",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Country data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "createCity" => {
                let input_de: CreateCityInput = serde_json::from_value(input)?;
                let db_result: CreateCityResult =
                    self.client.query("createCity", &input_de).await?;

                let matches = db_result.city.name == input_de.name
                    && db_result.city.description == input_de.description;

                if matches {
                    let msg = format!(
                        "City created!\n {}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, msg))
                } else {
                    let msg = format!(
                        "City data mismatch\n {}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, msg))
                }
            }
            _ => Ok((
                false,
                format!(
                    "Unknown query: '{}'. Check your query name in queries.hx",
                    query_name
                ),
            )),
        }
    }
}

impl ParsedSchema {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;
        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self, String> {
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();
        let mut vectors = HashMap::new();
        let mut lines = content.lines().map(str::trim);

        while let Some(line) = lines.next() {
            if let Some((schema_type, after_prefix)) = detect_schema_type(line) {
                if let Some(bracket_pos) = after_prefix.find('{') {
                    let node_name = after_prefix[..bracket_pos].trim();
                    let mut properties = HashSet::new();

                    if schema_type == "edge" {
                        let mut from_type = String::new();
                        let mut to_type = String::new();
                        let mut properties = HashSet::new();
                        let mut in_properties_block = false;

                        for prop_line in &mut lines {
                            if prop_line == "}" {
                                break;
                            }

                            let trimmed = prop_line.trim();
                            if trimmed.starts_with("Properties:") {
                                in_properties_block = true;
                                continue;
                            }

                            if let Some((field_name, field_value)) = trimmed.split_once(":") {
                                let name = field_name.trim();
                                let value = field_value.trim().trim_end_matches(",");

                                match name {
                                    "From" => {
                                        from_type = value.to_string();
                                    }
                                    "To" => {
                                        to_type = value.to_string();
                                    }
                                    _ if in_properties_block => {
                                        properties.insert(Property {
                                            name: name.to_string(),
                                            prop_type: value.to_string(),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                        let edge_info = EdgeInfo {
                            from_type,
                            to_type,
                            properties,
                        };

                        edges.insert(node_name.to_string(), edge_info);
                    } else {
                        for prop_line in &mut lines {
                            if prop_line == "}" {
                                break;
                            }
                            if let Some((prop_name, prop_type)) = prop_line.split_once(':') {
                                properties.insert(Property {
                                    name: prop_name.trim().to_string(),
                                    prop_type: prop_type.trim().to_string(),
                                });
                            }
                        }
                        match schema_type {
                            "node" => {
                                nodes.insert(node_name.to_string(), properties);
                            }
                            "vector" => {
                                vectors.insert(node_name.to_string(), properties);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        Ok(ParsedSchema {
            nodes,
            edges,
            vectors,
        })
    }

    pub fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
        let mut property_errors: HashMap<String, PropertyErrors> = HashMap::new();

        let users_answer: HashSet<&String> = self.nodes.keys().collect();
        let answer: HashSet<&String> = expected.nodes.keys().collect();

        let missing_nodes: Vec<String> = answer
            .difference(&users_answer)
            .map(|s| (*s).clone())
            .collect();
        let extra_nodes: Vec<String> = users_answer
            .difference(&answer)
            .map(|s| (*s).clone())
            .collect();

        let user_edges: HashSet<String> = self.edges.keys().cloned().collect();
        let expected_edges: HashSet<String> = expected.edges.keys().cloned().collect();

        let missing_edges: Vec<String> = expected_edges.difference(&user_edges).cloned().collect();
        let extra_edges: Vec<String> = user_edges.difference(&expected_edges).cloned().collect();
        let mut edge_errors: HashMap<String, EdgeErrors> = HashMap::new();

        let common_edges: Vec<String> = user_edges.intersection(&expected_edges).cloned().collect();
        let common_nodes: Vec<String> = users_answer
            .intersection(&answer)
            .map(|s| (*s).clone())
            .collect();

        for node in &common_nodes {
            let user_properties: &HashSet<Property> = &self.nodes[node];
            let expected_properties: &HashSet<Property> = &expected.nodes[node];

            if user_properties != expected_properties {
                let missing: Vec<String> = expected_properties
                    .difference(user_properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                let extra: Vec<String> = user_properties
                    .difference(expected_properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                let prop_errors = PropertyErrors {
                    missing,
                    extra,
                    wrong_type: Vec::new(),
                };

                property_errors.insert(node.clone(), prop_errors);
            }
        }

        for edge in &common_edges {
            let user_edge = &self.edges[edge];
            let expected_edge = &expected.edges[edge];

            let from_check = if user_edge.from_type != expected_edge.from_type {
                Some((user_edge.from_type.clone(), expected_edge.from_type.clone()))
            } else {
                None
            };

            let to_check = if user_edge.to_type != expected_edge.to_type {
                Some((user_edge.to_type.clone(), expected_edge.to_type.clone()))
            } else {
                None
            };

            let prop_errors = if user_edge.properties != expected_edge.properties {
                let missing: Vec<String> = expected_edge
                    .properties
                    .difference(&user_edge.properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                let extra: Vec<String> = user_edge
                    .properties
                    .difference(&expected_edge.properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                PropertyErrors {
                    missing,
                    extra,
                    wrong_type: Vec::new(),
                }
            } else {
                PropertyErrors {
                    missing: Vec::new(),
                    extra: Vec::new(),
                    wrong_type: Vec::new(),
                }
            };

            if from_check.is_some()
                || to_check.is_some()
                || !prop_errors.missing.is_empty()
                || !prop_errors.extra.is_empty()
            {
                edge_errors.insert(
                    edge.clone(),
                    EdgeErrors {
                        from_type_mismatch: from_check,
                        to_type_mismatch: to_check,
                        property_errors: prop_errors,
                    },
                );
            }
        }

        let user_vectors: HashSet<String> = self.vectors.keys().cloned().collect();
        let expected_vectors: HashSet<String> = expected.vectors.keys().cloned().collect();

        let missing_vectors: Vec<String> = expected_vectors
            .difference(&user_vectors)
            .cloned()
            .collect();
        let extra_vectors: Vec<String> = user_vectors
            .difference(&expected_vectors)
            .cloned()
            .collect();

        let mut vector_errors: HashMap<String, PropertyErrors> = HashMap::new();
        let common_vectors: Vec<String> = user_vectors
            .intersection(&expected_vectors)
            .cloned()
            .collect();

        for vector in &common_vectors {
            let user_properties: &HashSet<Property> = &self.vectors[vector];
            let expected_properties: &HashSet<Property> = &expected.vectors[vector];

            if user_properties != expected_properties {
                let missing: Vec<String> = expected_properties
                    .difference(user_properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                let extra: Vec<String> = user_properties
                    .difference(expected_properties)
                    .map(|prop| prop.name.clone())
                    .collect();

                let prop_errors = PropertyErrors {
                    missing,
                    extra,
                    wrong_type: Vec::new(),
                };

                vector_errors.insert(vector.clone(), prop_errors);
            }
        }

        ValidationResult {
            is_correct: missing_nodes.is_empty()
                && extra_nodes.is_empty()
                && property_errors.is_empty()
                && missing_edges.is_empty()
                && extra_edges.is_empty()
                && edge_errors.is_empty()
                && missing_vectors.is_empty()
                && extra_vectors.is_empty()
                && vector_errors.is_empty(),
            missing_nodes,
            extra_nodes,
            property_errors,
            missing_edges,
            extra_edges,
            edge_errors,
            missing_vectors,
            extra_vectors,
            vector_errors,
        }
    }
}

fn detect_schema_type(line: &str) -> Option<(&str, &str)> {
    if let Some(after) = line.strip_prefix("N::") {
        Some(("node", after))
    } else if let Some(after) = line.strip_prefix("E::") {
        Some(("edge", after))
    } else if let Some(after) = line.strip_prefix("V::") {
        Some(("vector", after))
    } else {
        None
    }
}

pub fn run_helix_check() -> bool {
    let output = Command::new("helix").arg("check").output();
    match output {
        Ok(result) => result.status.success(),
        Err(_) => {
            println!("Error: Could not run 'helix check'. Make sure HelixDB is installed.");
            false
        }
    }
}

pub fn check_helix_init() -> bool {
    Path::new("helixdb-cfg").exists()
}

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

pub fn save_created_entity(entity_type: &str, entity_data: &serde_json::Value) -> Result<(), String> {
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
    let instance_file = "helixdb-cfg/instance.txt";
    
    if let Ok(instance_id) = std::fs::read_to_string(instance_file) {
        let instance_id = instance_id.trim().to_string();
        
        let mut instance_data = load_instance_data();
        instance_data["instance_id"] = json!(instance_id.clone());
        let _ = save_instance_data(&instance_data);
        
        return Ok(instance_id);
    }
    
    println!("First time running queries! We need your HelixDB instance ID.");
    println!("Run 'helix instances' in another terminal and copy your instance ID.");
    println!("Enter your instance ID:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
    let instance_id = input.trim().to_string();
    
    std::fs::write(instance_file, &instance_id).map_err(|e| format!("Failed to save instance ID: {}", e))?;
    
    let mut instance_data = load_instance_data();
    instance_data["instance_id"] = json!(instance_id.clone());
    save_instance_data(&instance_data).map_err(|e| format!("Failed to save to instance.json: {}", e))?;
    
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
            
            if stdout_str.contains("No Helix instance found") || stderr_str.contains("No Helix instance found") {
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

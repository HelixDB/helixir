use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::path::Path;
use std::process::Command;

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

        let users_answer: HashSet<String> = self.nodes.keys().cloned().collect();
        let answer: HashSet<String> = expected.nodes.keys().cloned().collect();

        let missing_nodes: Vec<String> = answer.difference(&users_answer).cloned().collect();
        let extra_nodes: Vec<String> = users_answer.difference(&answer).cloned().collect();

        // HEY nothing is weird about the variable below alright?
        // just let the user edge brah
        let user_edges: HashSet<String> = self.edges.keys().cloned().collect();
        let expected_edges: HashSet<String> = expected.edges.keys().cloned().collect();

        let missing_edges: Vec<String> = expected_edges.difference(&user_edges).cloned().collect();
        let extra_edges: Vec<String> = user_edges.difference(&expected_edges).cloned().collect();
        let mut edge_errors: HashMap<String, EdgeErrors> = HashMap::new();

        let common_edges: Vec<String> = user_edges.intersection(&expected_edges).cloned().collect();
        let common_nodes: Vec<String> = users_answer.intersection(&answer).cloned().collect();

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

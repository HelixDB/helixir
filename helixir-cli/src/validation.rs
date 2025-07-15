use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    pub name: String,
    pub prop_type: String,
}

#[derive(Debug)]
pub struct ParsedSchema {
    pub nodes: HashMap<String, HashSet<Property>>,
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
}

impl ParsedSchema {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(file_path).expect("Err reading file");
        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self, String> {
        let mut nodes: HashMap<String, HashSet<Property>> = HashMap::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("N::") {
                let after_n = &line[3..];
                if let Some(bracket_pos) = after_n.find("{") {
                    let node_name = after_n[..bracket_pos].trim();
                    let mut properties: HashSet<Property> = HashSet::new();
                    i += 1;
                    while i < lines.len() {
                        let prop_line = lines[i].trim();
                        if prop_line == "}" {
                            break;
                        }

                        if prop_line.contains(":") {
                            let parts: Vec<&str> = prop_line.split(":").collect();
                            if parts.len() == 2 {
                                let prop_name = parts[0].trim();
                                let prop_type = parts[1].trim();
                                let property = Property {
                                    name: prop_name.to_string(),
                                    prop_type: prop_type.to_string(),
                                };
                                properties.insert(property);
                            }
                        }
                        i += 1;
                    }
                    nodes.insert(node_name.to_string(), properties);
                }
            }
            i += 1;
        }
        Ok(ParsedSchema { nodes })
    }

    pub fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
        let mut property_errors: HashMap<String, PropertyErrors> = HashMap::new();

        let users_answer: HashSet<String> = self.nodes.keys().cloned().collect();
        let answer: HashSet<String> = expected.nodes.keys().cloned().collect();

        let missing_nodes: Vec<String> = answer.difference(&users_answer).cloned().collect();
        let extra_nodes: Vec<String> = users_answer.difference(&answer).cloned().collect();

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

        ValidationResult {
            is_correct: missing_nodes.is_empty()
                && extra_nodes.is_empty()
                && property_errors.is_empty(),
            missing_nodes,
            extra_nodes,
            property_errors,
        }
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

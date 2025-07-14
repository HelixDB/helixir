use std::collections::{HashMap, HashSet};
use std::f32::consts::LN_10;
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

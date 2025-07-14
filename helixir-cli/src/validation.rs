use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    pub name: String,
    pub prop_type: String,
}

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
        // to do
    }

    fn parse(content: &str) -> Result<Self, String> {
        // to do
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

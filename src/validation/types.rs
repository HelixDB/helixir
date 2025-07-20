use std::collections::{HashMap, HashSet};

use helix_db::HelixDB;

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
    #[allow(dead_code)]
    pub wrong_type: Vec<(String, String, String)>,
}

#[derive(Debug)]
pub struct QueryValidator {
    pub(crate) client: HelixDB,
}

pub struct ValidationResult {
    pub is_correct: bool,
    pub missing_nodes: Vec<String>,
    #[allow(dead_code)]
    pub extra_nodes: Vec<String>,
    pub property_errors: HashMap<String, PropertyErrors>,
    pub missing_edges: Vec<String>,
    #[allow(dead_code)]
    pub extra_edges: Vec<String>,
    pub edge_errors: HashMap<String, EdgeErrors>,
    pub missing_vectors: Vec<String>,
    #[allow(dead_code)]
    pub extra_vectors: Vec<String>,
    pub vector_errors: HashMap<String, PropertyErrors>,
}

pub struct EdgeErrors {
    pub from_type_mismatch: Option<(String, String)>,
    pub to_type_mismatch: Option<(String, String)>,
    pub property_errors: PropertyErrors,
}

#[derive(Debug, Clone)]
pub struct ParsedQuery {
    #[allow(dead_code)]
    pub name: String,
    pub parameters: String,
    pub body: String,
}

#[derive(Debug)]
pub struct ParsedQueries {
    pub queries: HashMap<String, ParsedQuery>,
}

pub struct QueryValidationResult {
    pub is_correct: bool,
    pub missing_queries: Vec<String>,
    pub extra_queries: Vec<String>,
    pub query_errors: HashMap<String, String>,
}

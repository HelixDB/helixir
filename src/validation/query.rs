use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::validation::{ParsedQueries, ParsedQuery, QueryValidationResult};

impl ParsedQueries {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;
        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self, String> {
        let mut queries = HashMap::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() || line.starts_with("//") {
                i += 1;
                continue;
            }

            if line.starts_with("QUERY ") {
                if let Some(arrow_pos) = line.find(" =>") {
                    let query_def = &line[6..arrow_pos];

                    if let Some(paren_pos) = query_def.find('(') {
                        let query_name = query_def[..paren_pos].trim().to_string();
                        let params_end = query_def.rfind(')').unwrap_or(query_def.len());
                        let parameters = query_def[paren_pos + 1..params_end].trim().to_string();

                        let mut body_lines = Vec::new();
                        i += 1;

                        while i < lines.len() {
                            let body_line = lines[i].trim();
                            if body_line.is_empty() || body_line.starts_with("//") {
                                i += 1;
                                continue;
                            }
                            if body_line.starts_with("QUERY ") {
                                i -= 1;
                                break;
                            }
                            body_lines.push(body_line);
                            i += 1;
                        }

                        let body = body_lines.join("\n");
                        queries.insert(
                            query_name.clone(),
                            ParsedQuery {
                                name: query_name,
                                parameters,
                                body,
                            },
                        );
                    }
                }
            }
            i += 1;
        }

        Ok(ParsedQueries { queries })
    }

    pub fn validate_against(&self, expected: &ParsedQueries) -> QueryValidationResult {
        let mut query_errors = HashMap::new();

        let user_queries: HashSet<String> = self.queries.keys().cloned().collect();
        let expected_queries: HashSet<String> = expected.queries.keys().cloned().collect();

        let missing_queries: Vec<String> = expected_queries
            .difference(&user_queries)
            .cloned()
            .collect();
        let extra_queries: Vec<String> = Vec::new();

        for query_name in user_queries.intersection(&expected_queries) {
            let user_query = &self.queries[query_name];
            let expected_query = &expected.queries[query_name];

            let mut errors = Vec::new();

            if user_query.parameters != expected_query.parameters {
                errors.push(format!(
                    "Parameters mismatch. Expected: ({}), Got: ({})",
                    expected_query.parameters, user_query.parameters
                ));
            }

            let user_body_normalized = normalize_query_body(&user_query.body);
            let expected_body_normalized = normalize_query_body(&expected_query.body);

            if user_body_normalized != expected_body_normalized {
                errors.push("Query body differs from expected implementation".to_string());
            }

            if !errors.is_empty() {
                query_errors.insert(query_name.clone(), errors.join(". "));
            }
        }

        QueryValidationResult {
            is_correct: missing_queries.is_empty()
                && extra_queries.is_empty()
                && query_errors.is_empty(),
            missing_queries,
            extra_queries,
            query_errors,
        }
    }
}

fn normalize_query_body(body: &str) -> String {
    body.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

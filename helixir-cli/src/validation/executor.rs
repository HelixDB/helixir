use crate::lesson_types::*;
use crate::validation::{QueryValidator, get_latest_entity_id, save_created_entity};
use helix_db::{HelixDB, HelixDBClient};
use serde_json::json;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
enum EntityType {
    Continent,
    Country,
    City,
}

impl EntityType {
    fn storage_key(&self) -> &'static str {
        match self {
            Self::Continent => "continents",
            Self::Country => "countries", 
            Self::City => "cities",
        }
    }

    fn dependency_error(&self) -> &'static str {
        match self {
            Self::Continent => "No continent found. Please run lesson 5 first to create a continent.",
            Self::Country => "No country found. Please run lesson 6 first to create a country.",
            Self::City => "No city found. Please run lesson 7 first to create a city.",
        }
    }

    fn id_field(&self) -> &'static str {
        match self {
            Self::Continent => "continent_id",
            Self::Country => "country_id",
            Self::City => "city_id",
        }
    }
}

impl QueryValidator {
    pub fn new() -> Self {
        Self {
            client: HelixDB::new(None, None),
        }
    }

    async fn execute_query<I, R>(&self, query_name: &str, input: &I) -> anyhow::Result<R>
    where
        I: Serialize + Sync,
        R: DeserializeOwned,
    {
        self.client
            .query(query_name, input)
            .await
            .map_err(|e| anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e))
    }

    async fn execute_create_query<I, R>(
        &self,
        query_name: &str,
        input: serde_json::Value,
        storage_key: &str,
        validator: impl Fn(&I, &R) -> bool,
        storage_data_fn: impl Fn(&R) -> serde_json::Value,
        success_msg_fn: impl Fn(&R) -> String,
        error_msg_fn: impl Fn(&I, &R) -> String,
    ) -> anyhow::Result<(bool, String)>
    where
        I: DeserializeOwned + Serialize + Sync,
        R: DeserializeOwned + Serialize,
    {
        let input_de: I = serde_json::from_value(input)?;
        let db_result: R = self.execute_query(query_name, &input_de).await?;

        if validator(&input_de, &db_result) {
            let storage_data = storage_data_fn(&db_result);
            if let Err(e) = save_created_entity(storage_key, &storage_data) {
                println!("Warning: Could not save {} data: {}", storage_key, e);
            }
            Ok((true, success_msg_fn(&db_result)))
        } else {
            Ok((false, error_msg_fn(&input_de, &db_result)))
        }
    }

    async fn execute_get_query<I, R>(
        &self,
        query_name: &str,
        entity_type: EntityType,
        input: serde_json::Value,
        validator: impl Fn(&R) -> bool,
    ) -> anyhow::Result<(bool, String)>
    where
        I: DeserializeOwned + Serialize + Sync,
        R: DeserializeOwned + Serialize,
    {
        let entity_id = get_latest_entity_id(entity_type.storage_key())
            .ok_or_else(|| anyhow::anyhow!(entity_type.dependency_error()))?;

        let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
        input_obj[entity_type.id_field()] = json!(entity_id);

        let input_de: I = serde_json::from_value(input_obj)?;
        let db_result: R = self.execute_query(query_name, &input_de).await?;

        if validator(&db_result) {
            let success_msg = format!(
                "{} retrieved successfully!\nDatabase result:\n{}",
                query_name,
                serde_json::to_string_pretty(&db_result)?
            );
            Ok((true, success_msg))
        } else {
            let error_msg = format!(
                "{} retrieval failed or returned empty data\nDatabase result:\n{}",
                query_name,
                serde_json::to_string_pretty(&db_result)?
            );
            Ok((false, error_msg))
        }
    }

    pub async fn execute_and_compare(
        &self,
        query_name: &str,
        input: serde_json::Value,
    ) -> anyhow::Result<(bool, String)> {
        match query_name {
            "createContinent" => {
                self.execute_create_query::<AddContinentInput, AddContinentResult>(
                    query_name,
                    input,
                    "continents",
                    |input, result| result.continent.name == input.name,
                    |result| json!({
                        "id": result.continent.id,
                        "name": result.continent.name
                    }),
                    |result| format!(
                        "Continent created successfully!\nDatabase result:\n{}\nSaved continent ID for future lessons.",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                    |input, result| format!(
                        "Query executed but result doesn't match expected.\nDatabase returned:\n{}\nExpected name: '{}'",
                        serde_json::to_string_pretty(result).unwrap_or_default(),
                        input.name
                    ),
                ).await
            }
            "createCountry" => {
                let continent_id = get_latest_entity_id("continents").ok_or_else(|| {
                    anyhow::anyhow!("No continent found. Please run lesson 5 first to create a continent.")
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["continent_id"] = json!(continent_id);

                self.execute_create_query::<AddCountryInput, AddCountryResult>(
                    query_name,
                    input_obj,
                    "countries",
                    |input, result| {
                        result.country.name == input.name
                            && result.country.currency == input.currency
                            && result.country.population == input.population
                            && result.country.gdp == input.gdp
                    },
                    |result| json!({
                        "id": result.country.id,
                        "name": result.country.name,
                        "currency": result.country.currency,
                        "population": result.country.population,
                        "gdp": result.country.gdp,
                        "continent_id": continent_id
                    }),
                    |result| format!(
                        "Country created successfully!\nDatabase result:\n{}\nSaved country ID for future lessons.",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                    |_, result| format!(
                        "Country data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                ).await
            }
            "createCity" => {
                let country_id = get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!("No country found. Please run lesson 6 first to create a country.")
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["country_id"] = json!(country_id);

                self.execute_create_query::<AddCityInput, AddCityResult>(
                    query_name,
                    input_obj,
                    "cities",
                    |input, result| {
                        result.city.name == input.name && result.city.description == input.description
                    },
                    |result| json!({
                        "id": result.city.id,
                        "name": result.city.name,
                        "description": result.city.description,
                        "country_id": country_id
                    }),
                    |result| format!(
                        "City created successfully!\nDatabase result:\n{}\nSaved city ID for future lessons.",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                    |_, result| format!(
                        "City data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                ).await
            }
            "setCapital" => {
                let country_id = get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!("No country found. Please create a country first in previous lessons.")
                })?;

                let city_id = get_latest_entity_id("cities").ok_or_else(|| {
                    anyhow::anyhow!("No city found. Please create a city first in previous lessons.")
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["country_id"] = json!(country_id);
                input_obj["city_id"] = json!(city_id);

                let input_de: AddCapitalInput = serde_json::from_value(input_obj)?;
                let db_result: AddCapitalResult = self.execute_query("setCapital", &input_de).await?;

                let edge_matches = db_result.country_capital.from_node == country_id
                    && db_result.country_capital.to_node == city_id;

                if edge_matches {
                    let success_msg = format!(
                        "Capital relationship created successfully!\nDatabase result:\n{}\nCountry '{}' now has capital city '{}'.",
                        serde_json::to_string_pretty(&db_result)?,
                        country_id,
                        city_id
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Capital relationship mismatch\nExpected: from_node='{}', to_node='{}'\nGot: from_node='{}', to_node='{}'\nDatabase result:\n{}",
                        country_id,
                        city_id,
                        db_result.country_capital.from_node,
                        db_result.country_capital.to_node,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "embedDescription" => {
                let city_id = get_latest_entity_id("cities").ok_or_else(|| {
                    anyhow::anyhow!("No city found. Please create a city first in previous lessons.")
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["city_id"] = json!(city_id);

                let input_de: CreateDescEmbeddingInput = serde_json::from_value(input_obj)?;
                let db_result: CreateDescEmbeddingResult = self.execute_query("embedDescription", &input_de).await?;

                let vector_matches = !db_result.embedding.data.is_empty()
                    && db_result.embedding.data.len() == input_de.vector.len();

                if vector_matches {
                    let success_msg = format!(
                        "Embedding created successfully!\nDatabase result:\n{}\nCity '{}' now has description embedding with {} dimensions.",
                        serde_json::to_string_pretty(&db_result)?,
                        city_id,
                        db_result.embedding.data.len()
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Embedding vector mismatch\nExpected vector length: {}\nGot vector length: {}\nDatabase result:\n{}",
                        input_de.vector.len(),
                        db_result.embedding.data.len(),
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getContinent" => {
                self.execute_get_query::<GetContinentInput, GetContinentResult>(
                    query_name,
                    EntityType::Continent,
                    input,
                    |result| !result.continent.id.is_empty() && !result.continent.name.is_empty(),
                ).await
            }
            "getCountry" => {
                self.execute_get_query::<GetCountryInput, GetCountryResult>(
                    query_name,
                    EntityType::Country,
                    input,
                    |result| !result.country.id.is_empty() && !result.country.name.is_empty(),
                ).await
            }
            "getCity" => {
                self.execute_get_query::<GetCityInput, GetCityResult>(
                    query_name,
                    EntityType::City,
                    input,
                    |result| !result.city.id.is_empty() && !result.city.name.is_empty(),
                ).await
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
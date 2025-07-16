use crate::lesson_types::*;
use crate::validation::{QueryValidator, get_latest_entity_id, save_created_entity};
use helix_db::{HelixDB, HelixDBClient};
use serde_json::json;

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
    ) -> anyhow::Result<(bool, String)> {
        match query_name {
            "createContinent" => {
                let input_de: AddContinentInput = serde_json::from_value(input)?;
                let db_result: AddContinentResult = self
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
                let continent_id = get_latest_entity_id("continents").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No continent found. Please run lesson 5 first to create a continent."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["continent_id"] = json!(continent_id);

                let input_de: AddCountryInput = serde_json::from_value(input_obj)?;
                let db_result: AddCountryResult =
                    self.client.query("createCountry", &input_de).await?;

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
                let country_id = get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No country found. Please run lesson 6 first to create a country."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["country_id"] = json!(country_id);

                let input_de: AddCityInput = serde_json::from_value(input_obj)?;
                let db_result: AddCityResult = self.client.query("createCity", &input_de).await?;

                let matches = db_result.city.name == input_de.name
                    && db_result.city.description == input_de.description;

                if matches {
                    let city_data = json!({
                        "id": db_result.city.id,
                        "name": db_result.city.name,
                        "description": db_result.city.description,
                        "country_id": country_id
                    });

                    if let Err(e) = save_created_entity("cities", &city_data) {
                        println!("Warning: Could not save city data: {}", e);
                    }

                    let msg = format!(
                        "City created successfully!\nDatabase result:\n{}\nSaved city ID for future lessons.",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, msg))
                } else {
                    let msg = format!(
                        "City data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, msg))
                }
            }
            "setCapital" => {
                let country_id = get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No country found. Please create a country first in previous lessons."
                    )
                })?;

                let city_id = get_latest_entity_id("cities").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No city found. Please create a city first in previous lessons."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["country_id"] = json!(country_id);
                input_obj["city_id"] = json!(city_id);

                let input_de: AddCapitalInput = serde_json::from_value(input_obj)?;
                let db_result: AddCapitalResult =
                    self.client.query("setCapital", &input_de).await?;

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
                    anyhow::anyhow!(
                        "No city found. Please create a city first in previous lessons."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["city_id"] = json!(city_id);

                let input_de: CreateDescEmbeddingInput = serde_json::from_value(input_obj)?;
                let db_result: CreateDescEmbeddingResult = self
                    .client
                    .query("embedDescription", &input_de)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e)
                    })?;

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
                let continent_id = get_latest_entity_id("continents").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No continent found. Please run lesson 5 first to create a continent."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["continent_id"] = json!(continent_id);

                let input_de: GetContinentInput = serde_json::from_value(input_obj)?;
                let db_result: GetContinentResult = self
                    .client
                    .query("getContinent", &input_de)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e)
                    })?;

                let continent_exists = !db_result.continent.id.is_empty()
                    && !db_result.continent.name.is_empty();

                if continent_exists {
                    let success_msg = format!(
                        "Continent retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Continent retrieval failed or returned empty data\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountry" => {
                let country_id = get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No country found. Please run lesson 6 first to create a country."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["country_id"] = json!(country_id);

                let input_de: GetCountryInput = serde_json::from_value(input_obj)?;
                let db_result: GetCountryResult = self
                    .client
                    .query("getCountry", &input_de)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e)
                    })?;

                let country_exists = !db_result.country.id.is_empty()
                    && !db_result.country.name.is_empty();

                if country_exists {
                    let success_msg = format!(
                        "Country retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Country retrieval failed or returned empty data\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCity" => {
                let city_id = get_latest_entity_id("cities").ok_or_else(|| {
                    anyhow::anyhow!(
                        "No city found. Please run lesson 7 first to create a city."
                    )
                })?;

                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                input_obj["city_id"] = json!(city_id);

                let input_de: GetCityInput = serde_json::from_value(input_obj)?;
                let db_result: GetCityResult = self
                    .client
                    .query("getCity", &input_de)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Query failed: {}. Check your query name and syntax.", e)
                    })?;

                let city_exists = !db_result.city.id.is_empty()
                    && !db_result.city.name.is_empty();

                if city_exists {
                    let success_msg = format!(
                        "City retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "City retrieval failed or returned empty data\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
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

use crate::lesson_types::*;
use crate::validation::{QueryValidator, get_latest_entity_id, save_created_entity, load_instance_data};
use helix_rs::{HelixDB, HelixDBClient};
use serde_json::json;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

#[derive(Debug)]
enum EntityType {
    Continent,
    Country,
    City,
}

impl EntityType {
    #[allow(dead_code)]
    fn storage_key(&self) -> &'static str {
        match self {
            Self::Continent => "continents",
            Self::Country => "countries", 
            Self::City => "cities",
        }
    }

    #[allow(dead_code)]
    fn dependency_error(&self) -> &'static str {
        match self {
            Self::Continent => "No continent found. Please run lesson 5 first to create a continent.",
            Self::Country => "No country found. Please run lesson 6 first to create a country.",
            Self::City => "No city found. Please run lesson 7 first to create a city.",
        }
    }

    #[allow(dead_code)]
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
            client: HelixDB::new(Some("http://localhost:6969"), None, None),
        }
    }

    fn get_entity_mapping_for_lesson6() -> HashMap<&'static str, &'static str> {
        let mut mapping = HashMap::new();
        mapping.insert("London", "United Kingdom");
        mapping.insert("Manchester", "United Kingdom");
        mapping.insert("Berlin", "Germany");
        mapping.insert("Hamburg", "Germany");
        mapping
    }


    fn get_city_id_by_name(&self, city_name: &str) -> Option<String> {
        let instance_data = load_instance_data();
        if let Some(cities) = instance_data["created_entities"]["cities"].as_array() {
            for city in cities {
                if let Some(name) = city["name"].as_str() {
                    if name == city_name {
                        return city["id"].as_str().map(|s| s.to_string());
                    }
                }
            }
        }
        None
    }

    fn get_continent_id_by_name(&self, continent_name: &str) -> Option<String> {
        let instance_data = load_instance_data();
        if let Some(continents) = instance_data["created_entities"]["continents"].as_array() {
            for continent in continents {
                if let Some(name) = continent["name"].as_str() {
                    if name == continent_name {
                        return continent["id"].as_str().map(|s| s.to_string());
                    }
                }
            }
        }
        None
    }

    fn resolve_entity_id(&self, placeholder: &str, entity_type: &str) -> Result<String, anyhow::Error> {
        match placeholder {
            "europe_continent_id" | "some_continent_id" => {
                self.get_continent_id_by_name("Europe").ok_or_else(|| {
                    anyhow::anyhow!("Europe continent not found in instance data")
                })
            }
            "uk_country_id" => {
                self.get_country_id_by_name("United Kingdom").ok_or_else(|| {
                    anyhow::anyhow!("United Kingdom not found in instance data")
                })
            }
            "germany_country_id" => {
                self.get_country_id_by_name("Germany").ok_or_else(|| {
                    anyhow::anyhow!("Germany not found in instance data")
                })
            }
            "some_country_id" => {
                get_latest_entity_id("countries").ok_or_else(|| {
                    anyhow::anyhow!("No country found. Please create a country first.")
                })
            }
            "london_city_id" => {
                self.get_city_id_by_name("London").ok_or_else(|| {
                    anyhow::anyhow!("London not found in instance data")
                })
            }
            "berlin_city_id" => {
                self.get_city_id_by_name("Berlin").ok_or_else(|| {
                    anyhow::anyhow!("Berlin not found in instance data")
                })
            }
            "manchester_city_id" => {
                self.get_city_id_by_name("Manchester").ok_or_else(|| {
                    anyhow::anyhow!("Manchester not found in instance data")
                })
            }
            "hamburg_city_id" => {
                self.get_city_id_by_name("Hamburg").ok_or_else(|| {
                    anyhow::anyhow!("Hamburg not found in instance data")
                })
            }
            "ID" => {
                get_latest_entity_id(entity_type).ok_or_else(|| {
                    anyhow::anyhow!("No {} found. Please create one first.", entity_type)
                })
            }
            _ => {
                get_latest_entity_id(entity_type).ok_or_else(|| {
                    anyhow::anyhow!("No {} found for placeholder '{}'", entity_type, placeholder)
                })
            }
        }
    }

    fn replace_placeholder_ids(&self, mut input_obj: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
        if let Some(placeholder) = input_obj["continent_id"].as_str() {
            let real_id = self.resolve_entity_id(placeholder, "continents")?;
            input_obj["continent_id"] = json!(real_id);
        }
        
        if let Some(placeholder) = input_obj["country_id"].as_str() {
            let real_id = self.resolve_entity_id(placeholder, "countries")?;
            input_obj["country_id"] = json!(real_id);
        }
        
        if let Some(placeholder) = input_obj["city_id"].as_str() {
            let real_id = self.resolve_entity_id(placeholder, "cities")?;
            input_obj["city_id"] = json!(real_id);
        }

        Ok(input_obj)
    }

    fn get_country_id_by_name(&self, country_name: &str) -> Option<String> {
        let instance_data = load_instance_data();
        if let Some(countries) = instance_data["created_entities"]["countries"].as_array() {
            for country in countries {
                if let Some(name) = country["name"].as_str() {
                    if name == country_name {
                        return country["id"].as_str().map(|s| s.to_string());
                    }
                }
            }
        }
        None
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
        _entity_type: EntityType,
        input: serde_json::Value,
        validator: impl Fn(&R) -> bool,
    ) -> anyhow::Result<(bool, String)>
    where
        I: DeserializeOwned + Serialize + Sync,
        R: DeserializeOwned + Serialize,
    {
        let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
        let input_obj = self.replace_placeholder_ids(input_obj)?;

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
                        "Continent created successfully!\nDatabase result:\n{}",
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
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;

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
                    |result| {
                        let continent_id = get_latest_entity_id("continents").unwrap_or_default();
                        json!({
                            "id": result.country.id,
                            "name": result.country.name,
                            "currency": result.country.currency,
                            "population": result.country.population,
                            "gdp": result.country.gdp,
                            "continent_id": continent_id
                        })
                    },
                    |result| format!(
                        "Country created successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                    |_, result| format!(
                        "Country data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                ).await
            }
            "createCity" => {
                let mut input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                
                let city_name = input_obj["name"].as_str().unwrap_or("");
                let city_to_country_mapping = Self::get_entity_mapping_for_lesson6();
                
                let country_id = if let Some(country_name) = city_to_country_mapping.get(city_name) {
                    self.get_country_id_by_name(country_name).ok_or_else(|| {
                        anyhow::anyhow!("Country '{}' not found for city '{}'. Please ensure countries are created first.", country_name, city_name)
                    })?
                } else {
                    get_latest_entity_id("countries").ok_or_else(|| {
                        anyhow::anyhow!("No country found. Please run lesson 6 first to create a country.")
                    })?
                };

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
                        "City created successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                    |_, result| format!(
                        "City data mismatch\nDatabase result:\n{}",
                        serde_json::to_string_pretty(result).unwrap_or_default()
                    ),
                ).await
            }
            "setCapital" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();
                let city_id = input_obj["city_id"].as_str().unwrap().to_string();

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
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let city_id = input_obj["city_id"].as_str().unwrap().to_string();

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
            "getCapital" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;

                let input_de: GetCapitalInput = serde_json::from_value(input_obj)?;
                
                let db_result: GetCapitalResult = self.execute_query(query_name, &input_de).await?;

                if !db_result.capital.is_empty() && 
                   !db_result.capital[0].id.is_empty() && 
                   !db_result.capital[0].name.is_empty() {
                    let success_msg = format!(
                        "Capital retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Capital retrieval failed or returned empty data\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getAllContinents" => {
                let db_result: GetAllContinentsResult = self.execute_query(query_name, &serde_json::json!({})).await?;
                
                if !db_result.continents.is_empty() {
                    let success_msg = format!(
                        "All continents retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No continents found or query failed\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getAllCountries" => {
                let db_result: GetAllCountriesResult = self.execute_query(query_name, &serde_json::json!({})).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "All countries retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found or query failed\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getAllCities" => {
                let db_result: GetAllCitiesResult = self.execute_query(query_name, &serde_json::json!({})).await?;
                
                if !db_result.cities.is_empty() {
                    let success_msg = format!(
                        "All cities retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No cities found or query failed\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesInContinent" => {
                self.execute_get_query::<GetCountriesInContinentInput, GetCountriesInContinentResult>(
                    query_name,
                    EntityType::Continent,
                    input,
                    |result| !result.countries.is_empty(),
                ).await
            }
            "getCitiesInCountry" => {
                self.execute_get_query::<GetCitiesInCountryInput, GetCitiesInCountryResult>(
                    query_name,
                    EntityType::Country,
                    input,
                    |result| !result.cities.is_empty(),
                ).await
            }
            "getCountryNames" => {
                let raw_response: serde_json::Value = self.execute_query(query_name, &serde_json::json!({})).await?;
                
                let has_valid_data = if let Some(countries) = raw_response.get("countries") {
                    if let Some(countries_array) = countries.as_array() {
                        !countries_array.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if has_valid_data {
                    let success_msg = format!(
                        "Country names retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&raw_response)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found or query failed\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&raw_response)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getContinentByName" => {
                let input_de: GetContinentByNameInput = serde_json::from_value(input)?;
                let db_result: GetContinentByNameResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.continent.is_empty() && !db_result.continent[0].id.is_empty() {
                    let success_msg = format!(
                        "Continent retrieved by name successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No continent found with that name\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountryByName" => {
                let input_de: GetCountryByNameInput = serde_json::from_value(input)?;
                let db_result: GetCountryByNameResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.country.is_empty() && !db_result.country[0].id.is_empty() {
                    let success_msg = format!(
                        "Country retrieved by name successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No country found with that name\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCityByName" => {
                let input_de: GetCityByNameInput = serde_json::from_value(input)?;
                let db_result: GetCityByNameResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.city.is_empty() && !db_result.city[0].id.is_empty() {
                    let success_msg = format!(
                        "City retrieved by name successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No city found with that name\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesByCurrency" => {
                let input_de: GetCountriesByCurrencyInput = serde_json::from_value(input)?;
                let db_result: GetCountriesByCurrencyResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries filtered by currency successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found with currency '{}'\nDatabase result:\n{}",
                        input_de.currency,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesByPopulation" => {
                let input_de: GetCountriesByPopulationInput = serde_json::from_value(input)?;
                let db_result: GetCountriesByPopulationResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries filtered by population successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found with population less than {}\nDatabase result:\n{}",
                        input_de.max_population,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesByGdp" => {
                let input_de: GetCountriesByGdpInput = serde_json::from_value(input)?;
                let db_result: GetCountriesByGdpResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries filtered by GDP successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found with GDP greater than or equal to {}\nDatabase result:\n{}",
                        input_de.min_gdp,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesByPopGdp" => {
                let input_de: GetCountriesByPopGdpInput = serde_json::from_value(input)?;
                let db_result: GetCountriesByPopGdpResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries filtered by population and GDP successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found with population > {} and GDP <= {}\nDatabase result:\n{}",
                        input_de.min_population,
                        input_de.max_gdp,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountriesByCurrPop" => {
                let input_de: GetCountriesByCurrPopInput = serde_json::from_value(input)?;
                let db_result: GetCountriesByCurrPopResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries filtered by currency or population successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries found with currency '{}' or population <= {}\nDatabase result:\n{}",
                        input_de.currency,
                        input_de.max_population,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getContinentCities" => {
                let input_de: GetContinentCitiesInput = serde_json::from_value(input)?;
                let db_result: GetContinentCitiesResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.cities.is_empty() {
                    let success_msg = format!(
                        "Cities retrieved successfully from continent '{}'!\nDatabase result:\n{}",
                        input_de.continent_name,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No cities found in continent '{}'\nDatabase result:\n{}",
                        input_de.continent_name,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "countCapitals" => {
                match self.client.query::<serde_json::Value, serde_json::Value>(query_name, &serde_json::json!({})).await {
                    Ok(raw_response) => {
                        let success_msg = format!(
                            "Capital count retrieved successfully!\nDatabase result:\n{}",
                            serde_json::to_string_pretty(&raw_response).unwrap_or_default()
                        );
                        Ok((true, success_msg))
                    },
                    Err(e) => Ok((false, format!("Query failed: {}", e)))
                }
            }
            "getCountriesWithCapitals" => {
                let db_result: GetCountriesWithCapitalsResult = self.execute_query(query_name, &serde_json::json!({})).await?;
                
                if !db_result.countries.is_empty() {
                    let success_msg = format!(
                        "Countries with capitals retrieved successfully!\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No countries with capitals found or query failed\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "getCountryByCityCnt" => {
                let input_de: GetCountryByCityCntInput = serde_json::from_value(input)?;
                
                match self.client.query::<GetCountryByCityCntInput, serde_json::Value>(query_name, &input_de).await {
                    Ok(raw_response) => {
                        let success_msg = format!(
                            "Countries filtered by city count successfully!\nDatabase result:\n{}",
                            serde_json::to_string_pretty(&raw_response).unwrap_or_default()
                        );
                        Ok((true, success_msg))
                    },
                    Err(e) => Ok((false, format!("Query failed: {}", e)))
                }
            }
            "searchDescriptions" => {
                let input_de: SearchDescriptionsInput = serde_json::from_value(input)?;
                let db_result: SearchDescriptionsResult = self.execute_query(query_name, &input_de).await?;
                
                if !db_result.cities.is_empty() {
                    let success_msg = format!(
                        "Semantic search completed successfully!\nDatabase result:\n{}\nFound {} semantically similar cities for the search vector.",
                        serde_json::to_string_pretty(&db_result)?,
                        db_result.cities.len()
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "No cities found for semantic search\nDatabase result:\n{}",
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "updateCurrency" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();

                let input_de: UpdateCurrencyInput = serde_json::from_value(input_obj)?;
                let db_result: UpdateCurrencyResult = self.execute_query("updateCurrency", &input_de).await?;

                let currency_matches = db_result.country.currency == input_de.currency 
                    && db_result.country.id == country_id;

                if currency_matches {
                    let success_msg = format!(
                        "Country currency updated successfully!\nDatabase result:\n{}\nCountry '{}' currency updated to '{}'.",
                        serde_json::to_string_pretty(&db_result)?,
                        country_id,
                        input_de.currency
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Currency update mismatch\nExpected: currency='{}'\nGot: currency='{}'\nDatabase result:\n{}",
                        input_de.currency,
                        db_result.country.currency,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "updatePopGdp" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();

                let input_de: UpdatePopGdpInput = serde_json::from_value(input_obj)?;
                let db_result: UpdatePopGdpResult = self.execute_query("updatePopGdp", &input_de).await?;

                let values_match = db_result.country.population == input_de.population 
                    && db_result.country.gdp == input_de.gdp
                    && db_result.country.id == country_id;

                if values_match {
                    let success_msg = format!(
                        "Country population and GDP updated successfully!\nDatabase result:\n{}\nCountry '{}' population updated to {} and GDP updated to {}.",
                        serde_json::to_string_pretty(&db_result)?,
                        country_id,
                        input_de.population,
                        input_de.gdp
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Population/GDP update mismatch\nExpected: population={}, gdp={}\nGot: population={}, gdp={}\nDatabase result:\n{}",
                        input_de.population,
                        input_de.gdp,
                        db_result.country.population,
                        db_result.country.gdp,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "updateCapital" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();
                let city_id = input_obj["city_id"].as_str().unwrap().to_string();

                let input_de: UpdateCapitalInput = serde_json::from_value(input_obj)?;
                let db_result: UpdateCapitalResult = self.execute_query("updateCapital", &input_de).await?;

                let capital_updated = db_result.city.id == city_id;
                if capital_updated {
                    let success_msg = format!(
                        "Capital relationship updated successfully!\nDatabase result:\n{}\nCountry '{}' now has '{}' as its capital city.",
                        serde_json::to_string_pretty(&db_result)?,
                        country_id,
                        city_id
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Capital update mismatch\nExpected city ID: '{}'\nGot city ID: '{}'\nDatabase result:\n{}",
                        city_id,
                        db_result.city.id,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "updateDescription" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let city_id = input_obj["city_id"].as_str().unwrap().to_string();

                let input_de: UpdateDescriptionInput = serde_json::from_value(input_obj)?;
                let db_result: UpdateDescriptionResult = self.execute_query("updateDescription", &input_de).await?;

                let description_updated = db_result.city.description == input_de.description 
                    && db_result.city.id == city_id;

                if description_updated {
                    let success_msg = format!(
                        "Description and embedding updated successfully!\nDatabase result:\n{}\nCity '{}' description updated to '{}' with new vector embedding.",
                        serde_json::to_string_pretty(&db_result)?,
                        city_id,
                        input_de.description
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Description update mismatch\nExpected description: '{}'\nGot description: '{}'\nDatabase result:\n{}",
                        input_de.description,
                        db_result.city.description,
                        serde_json::to_string_pretty(&db_result)?
                    );
                    Ok((false, error_msg))
                }
            }
            "deleteCity" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let city_id = input_obj["city_id"].as_str().unwrap().to_string();

                let input_de: DeleteCityInput = serde_json::from_value(input_obj)?;

                let db_result: DeleteCityResult = self.execute_query("deleteCity", &input_de).await?;

                println!("db_result: {:?}", db_result);
                if db_result.success == "success" {
                    let success_msg = format!(
                        "City deleted successfully!\nDatabase result: \"{}\"\nCity '{}' has been removed from the graph.",
                        db_result.success,
                        city_id
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "City deletion failed: {}.\n\nTrying to delete city with ID: '{}'\n\nThis could be due to:\n1. HelixDB having issues with DELETE operations\n2. The city has dependencies (like being a capital) that prevent deletion\n3. Response format mismatch\n\nNote: The validation uses the latest created city from instance.json",
                        db_result.success,
                        city_id
                    );
                    Ok((false, error_msg))
                }
            }
            "deleteCapital" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();

                let input_de: DeleteCapitalInput = serde_json::from_value(input_obj)?;
                
                let db_result: DeleteCapitalResult = self.execute_query("deleteCapital", &input_de).await?;

                if db_result.success == "success" {
                    let success_msg = format!(
                        "Capital relationship deleted successfully!\nDatabase result: \"{}\"\nCountry '{}' no longer has a capital city relationship.",
                        db_result.success,
                        country_id
                    );
                    Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                        "Capital deletion failed: {}.\n\nTrying to delete capital relationship for country ID: '{}'\n\nThis could be due to:\n1. HelixDB having issues with DELETE operations\n2. The country has dependencies (like having cities) that prevent deletion\n3. Response format mismatch\n\nNote: The validation uses the latest created country from instance.json",
                        db_result.success,
                        country_id
                    );
                    Ok((false, error_msg))
                }
            }
            "deleteCountry" => {
                let input_obj = serde_json::from_value::<serde_json::Value>(input)?;
                let input_obj = self.replace_placeholder_ids(input_obj)?;
                
                let country_id = input_obj["country_id"].as_str().unwrap().to_string();

                let input_de: DeleteCountryInput = serde_json::from_value(input_obj)?;
                
                let db_result: DeleteCountryResult = self.execute_query("deleteCountry", &input_de).await?;
                if db_result.success == "success" {
                    let success_msg = format!(
                        "Country deleted successfully!\nDatabase result: \"{}\"\nCountry '{}' has been removed from the graph.",
                        db_result.success,
                            country_id
                        );
                        Ok((true, success_msg))
                } else {
                    let error_msg = format!(
                            "Country deletion failed: {}.\n\nTrying to delete country ID: '{}'\n\nNote: HelixDB appears to have issues with DELETE operations",
                            db_result.success,
                            country_id
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
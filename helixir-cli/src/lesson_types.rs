use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddContinentInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddContinentOutput {
    id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateContinentResult {
    pub continent: ContinentData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinentData {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCountryInput {
    pub continent_id: String,
    pub name: String,
    pub currency: String,
    pub population: u64,
    pub gdp: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCountryResult {
    pub country: CountryData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryData {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub population: u64,
    pub gdp: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCityInput {
    pub country_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCityResult {
    pub city: CityData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityData {
    pub id: String,
    pub name: String,
    pub description: String,
}

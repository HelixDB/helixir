use serde::{Deserialize, Serialize};

// continent lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct AddContinentInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddContinentResult {
    pub continent: ContinentData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinentData {
    pub id: String,
    pub name: String,
}

// country lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCountryInput {
    pub continent_id: String,
    pub name: String,
    pub currency: String,
    pub population: u64,
    pub gdp: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCountryResult {
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


// city lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCityInput {
    pub country_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCityResult {
    pub city: CityData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityData {
    pub id: String,
    pub name: String,
    pub description: String,
}

// set capital city lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCapitalInput {
    pub country_id: String,
    pub city_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCapitalResult {
    pub country_capital: CapitalEdgeData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalEdgeData {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    pub label: String,
}

// embedding lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDescEmbeddingInput {
    pub city_id: String,
    pub vector: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDescEmbeddingResult {
    pub embedding: DescEmbeddingData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescEmbeddingData {
    pub id: String,
    pub data: Vec<f64>,
    pub label: String,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityEmbeddingEdgeData {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    pub label: String,
}

// get continent, city and country

#[derive(Serialize, Deserialize, Debug)]
pub struct GetContinentInput {
    pub continent_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetContinentResult {
    pub continent: ContinentData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountryInput {
    pub country_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountryResult {
    pub country: CountryData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCityInput {
    pub city_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCityResult {
    pub city: CityData,
}

// get all by node

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllContinentsResult {
    pub continents: Vec<ContinentData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllCountriesResult {
    pub countries: Vec<CountryData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllCitiesResult {
    pub cities: Vec<CityData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountriesInContinentInput {
    pub continent_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountriesInContinentResult {
    pub countries: Vec<CountryData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCitiesInCountryInput {
    pub country_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCitiesInCountryResult {
    pub cities: Vec<CityData>,
}

// get capital city lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCapitalInput {
    pub country_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCapitalResult {
    pub capital: Vec<CityData>,
}

// get country names lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryNameData {
    pub name: Vec<String>,
    pub population: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountryNamesResult {
    pub countries: Vec<CountryNameData>,
}

// get nodes by name lesson

#[derive(Serialize, Deserialize, Debug)]
pub struct GetContinentByNameInput {
    pub continent_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetContinentByNameResult {
    pub continent: Vec<ContinentData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountryByNameInput {
    pub country_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCountryByNameResult {
    pub country: Vec<CountryData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCityByNameInput {
    pub city_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCityByNameResult {
    pub city: Vec<CityData>,
}


use serde::{Deserialize, Serialize};

// TO DO: Define lesson 1 structs (continent creation)
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

N::Continent {
    name: String
}

N::Country {
    name: String,
    currency: String,
    population: I64,
    gdp: F64
}

N::City {
    name: String,
    description: String,
    zip_codes: [String]
}

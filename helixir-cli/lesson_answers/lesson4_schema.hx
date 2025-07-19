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

E::Continent_to_Country {
    From: Continent,
    To: Country,
    Properties: {
    }
}

E::Country_to_City {
    From: Country,
    To: City,
    Properties: {
    }
}

E::Country_to_Capital {
    From: Country,
    To: City,
    Properties: {
    }
}

V::CityDescription {
    vector: [F64]
}

E::City_to_Embedding {
    From: City,
    To: CityDescription,
    Properties: {
    }
}

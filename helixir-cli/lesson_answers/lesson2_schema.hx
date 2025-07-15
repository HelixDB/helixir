N::Continent {
    name: String
}

N::Country {
    name: String,
    currency: String
}

N::City {
    name: String,
    description: String
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
QUERY createContinent (name: String) =>
    continent <- AddN<Continent>({name: name})
    RETURN continent

QUERY createCountry (continent_id: ID, name: String, currency: String, population: I64, gdp: F64) =>
    country <- AddN<Country>({name: name, currency: currency, population: population, gdp: gdp})
    continent <- N<Continent>(continent_id)
    continent_country <- AddE<Continent_to_Country>()::From(continent)::To(country)
    RETURN country
    
QUERY createCity (country_id: ID, name: String, description: String) =>
    city <- AddN<City>({name: name, description: description})
    country <- N<Country>(country_id)
    country_city <- AddE<Country_to_City>()::From(country)::To(city)
    RETURN city

QUERY setCapital (country_id: ID, city_id: ID) =>
    country <- N<Country>(country_id)
    city <- N<City>(city_id)
    country_capital <- AddE<Country_to_Capital>()::From(country)::To(city)
    RETURN country_capital

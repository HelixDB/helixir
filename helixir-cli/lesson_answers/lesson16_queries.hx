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

QUERY embedDescription (city_id: ID, vector: [F64]) =>
    embedding <- AddV<CityDescription>(vector)
    city <- N<City>(city_id)
    city_embedding <- AddE<City_to_Embedding>()::From(city)::To(embedding)
    RETURN embedding

QUERY getContinent (continent_id: ID) =>
    continent <- N<Continent>(continent_id)
    RETURN continent

QUERY getCountry (country_id: ID) =>
    country <- N<Country>(country_id)
    RETURN country

QUERY getCity (city_id: ID) =>
    city <- N<City>(city_id)
    RETURN city

QUERY getAllContinents () =>
    continents <- N<Continent>
    RETURN continents

QUERY getAllCountries () =>
    countries <- N<Country>
    RETURN countries

QUERY getAllCities () =>
    cities <- N<City>
    RETURN cities

QUERY getCountriesInContinent (continent_id: ID) =>
    continent <- N<Continent>(continent_id)
    countries <- continent::Out<Continent_to_Country>
    RETURN countries

QUERY getCitiesInCountry (country_id: ID) =>
    country <- N<Country>(country_id)
    cities <- country::Out<Country_to_City>
    RETURN cities

QUERY getCapital (country_id: ID) =>
    country <- N<Country>(country_id)
    capital <- country::Out<Country_to_Capital>
    RETURN capital

QUERY getCountryNames () =>
    countries <- N<Country>::{name}
    RETURN countries

QUERY getContinentByName (continent_name: String) =>
    continent <- N<Continent>::WHERE(_::{name}::EQ(continent_name))
    RETURN continent

QUERY getCountryByName (country_name: String) =>
    country <- N<Country>::WHERE(_::{name}::EQ(country_name))
    RETURN country
    
QUERY getCityByName (city_name: String) =>
    city <- N<City>::WHERE(_::{name}::EQ(city_name))
    RETURN city

QUERY getCountriesByCurrency (currency: String) =>
    countries <- N<Country>::WHERE(_::{currency}::EQ(currency))
    RETURN countries

QUERY getCountriesByPopulation (max_population: I64) =>
    countries <- N<Country>::WHERE(_::{population}::LT(max_population))
    RETURN countries

QUERY getCountriesByGdp (min_gdp: F64) =>
    countries <- N<Country>::WHERE(_::{gdp}::GTE(min_gdp))
    RETURN countries

QUERY getCountriesByPopGdp (min_population: I64, max_gdp: F64) =>
    countries <- N<Country>::WHERE(
                    AND(
                        _::{population}::GT(min_population),
                        _::{gdp}::LTE(max_gdp)
                    )
            )
    RETURN countries

QUERY getCountriesByCurrPop (currency: String, max_population: I64) =>
    countries <- N<Country>::WHERE(
                    OR(
                            _::{currency}::EQ(currency),
                            _::{population}::LTE(max_population)
                    )
            )
    RETURN countries

QUERY getCountriesWithCapitals () =>
    countries <- N<Country>::WHERE(EXISTS(_::Out<Country_to_Capital>))
    RETURN countries
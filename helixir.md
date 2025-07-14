# Quick Start

> In this short example, you will create a HelixDB instance, write a schema and queries, and run basic queries using the Python SDK.

Install the HelixCLI using the following command:

```bash
curl -sSL https://install.helix-db.com | bash
```

### Add Helix to your PATH

```bash
# For Unix (macOS, Linux)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
source ~/.zshrc
```

```bash
# For Bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
source ~/.bashrc
```

### Verify Installation

To verify that HelixCLI is installed correctly:

```bash
helix --version
```

Install the Helix container using the following command:

```bash
helix install
```

### Setup a new project

Setup a new project using the following command:

```bash
helix init --path helixdb-cfg
```

### Setup Your SDK

```bash
pip install helix-py
```

Click [here](../sdks/helix-py) for more information about the Python SDK.

## Design Your Schema

We will be using HelixDB to model the relationships between continents, countries, and cities.

Below is an example of the schema we will be using.

Write your schema in the newly created `schema.hx` file in the `helixdb-cfg` directory.

```rust
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

E::Country_to_Capital {
    From: Country,
    To: City,
    Properties: {
    }
}
```

## Creation Queries

Write your creation queries in the `query.hx` file in the `helixdb-cfg` directory.

```rust
QUERY createContinent (name: String) =>
    continent <- AddN<Continent>({name: name})
    RETURN continent

QUERY createCountry (continent_id: ID, name: String, currency: String) =>
    country <- AddN<Country>({name: name, currency: currency})
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
```

## Read Queries

Write your read queries in the `query.hx` file in the `helixdb-cfg` directory.

```rust
QUERY getAllContinents () =>
    continents <- N<Continent>
    RETURN continents

QUERY getAllCountries (continent_id: ID) =>
    continent <- N<Continent>(continent_id)
    countries <- continent::Out<Continent_to_Country>
    RETURN countries

QUERY getAllCities (country_id: ID) =>
    country <- N<Country>(country_id)
    cities <- country::Out<Country_to_City>
    RETURN cities

QUERY getCapital (country_id: ID) =>
    country <- N<Country>(country_id)
    capital <- country::Out<Country_to_Capital>
    RETURN capital
```

## Check Your Schema and Queries

Using the following command:

```bash
helix check
```

If you see
"Helix-QL schema and queries validated successfully with zero errors",
you are ready to deploy your instance!

## Deploy Instance

Using the following command:

```bash
helix deploy
```

If you see
"Successfully started Helix instance", you are ready to run queries!

## Import the SDK

```python
from helix.client import Client

client = Client(local=True, port=6969)
```

Click [here](../sdks/helix-py) for more information about the Python SDK.

## Data for continents, countries, and cities

```python
continent_data = [
    {"name": "Asia"},
    {"name": "North America"},
    {"name": "Europe"},
    {"name": "South America"}
]

europe = [
    {'name':'UK', 'currency':'Pounds'},
    {'name':'Germany', 'currency':'Euro'},
    {'name':'Italy', 'currency':'Euro'},
]

asia = [
    {'name':'China', 'currency':'Yuan'},
    {'name':'Japan', 'currency':'Yen'},
]

north_america = [
    {'name':'USA', 'currency':'Dollar'},
    {'name':'Mexico', 'currency':'Peso'}
]

south_america = [
    {'name':'Brazil', 'currency':'Real'},
    {'name':'Argentina', 'currency':'Peso'}
]

uk = [
    {"name": "London", "description": "Capital city known for its finance, culture, and history"},
    {"name": "Manchester", "description": "Northern hub for music, sports, and industry"},
    {"name": "Birmingham", "description": "Second-largest city, known for manufacturing and diversity"},
    {"name": "Liverpool", "description": "Port city famous for The Beatles and football"},
    {"name": "Bristol", "description": "Historic maritime city with a creative edge"}
]

germany = [
    {"name": "Berlin", "description": "Capital city known for history, culture, and politics"},
    {"name": "Munich", "description": "Bavarian capital known for Oktoberfest and tech innovation"},
    {"name": "Frankfurt", "description": "Major financial center and home to the ECB"},
    {"name": "Hamburg", "description": "Large port city with maritime economy and media hubs"},
    {"name": "Cologne", "description": "Cultural city known for its Gothic cathedral and trade fairs"}
]

italy = [
    {"name": "Rome", "description": "Capital city known for ancient history and architecture"},
    {"name": "Milan", "description": "Fashion and financial capital of northern Italy"},
    {"name": "Naples", "description": "Historic southern city and birthplace of pizza"},
    {"name": "Florence", "description": "Renaissance city known for art and museums"},
    {"name": "Venice", "description": "Famous canal city built on water with unique charm"}
]

china = [
    {"name": "Beijing", "description": "Capital city with political, cultural, and historical importance"},
    {"name": "Shanghai", "description": "Global financial hub and largest city by population"},
    {"name": "Shenzhen", "description": "Tech powerhouse and innovation center near Hong Kong"},
    {"name": "Guangzhou", "description": "Major trade and manufacturing city in southern China"},
    {"name": "Chengdu", "description": "Economic center of western China and home of pandas"}
]

japan = [
    {"name": "Tokyo", "description": "Capital city and global center for finance, tech, and culture"},
    {"name": "Osaka", "description": "Major economic hub known for food and commerce"},
    {"name": "Kyoto", "description": "Historic city famous for temples, shrines, and tradition"},
    {"name": "Yokohama", "description": "Port city just south of Tokyo with modern skyline"},
    {"name": "Nagoya", "description": "Industrial center and home to automotive manufacturing"}
]

usa = [
    {"name": "Washington, D.C.", "description": "Capital of the United States and seat of government"},
    {"name": "New York City", "description": "Largest city, a global hub for finance and culture"},
    {"name": "Los Angeles", "description": "Entertainment capital and second-largest city"},
    {"name": "Chicago", "description": "Major Midwest city known for architecture and industry"},
    {"name": "San Francisco", "description": "Tech-driven city and gateway to Silicon Valley"}
]

mexico = [
    {"name": "Mexico City", "description": "Capital and largest city, political and cultural center"},
    {"name": "Guadalajara", "description": "Major tech and cultural hub in western Mexico"},
    {"name": "Monterrey", "description": "Industrial and business center in the north"},
    {"name": "Puebla", "description": "Historic city known for colonial architecture and cuisine"},
    {"name": "Tijuana", "description": "Border city with strong ties to the U.S. and growing economy"}
]

brazil = [
    {"name": "Brasília", "description": "Modern capital and center of government"},
    {"name": "São Paulo", "description": "Largest city and financial powerhouse of Brazil"},
    {"name": "Rio de Janeiro", "description": "Famous for beaches, Carnival, and Christ the Redeemer"},
    {"name": "Belo Horizonte", "description": "Major city in mining and industry"},
    {"name": "Porto Alegre", "description": "Cultural and economic hub in southern Brazil"}
]

argentina = [
    {"name": "Buenos Aires", "description": "Capital city and cultural, political, and economic center"},
    {"name": "Córdoba", "description": "University city and industrial hub in central Argentina"},
    {"name": "Rosario", "description": "Major river port and birthplace of Che Guevara"},
    {"name": "Mendoza", "description": "Wine capital located at the foothills of the Andes"},
    {"name": "San Miguel de Tucumán", "description": "Historic city known for Argentine independence"}
]
```

## Calling Creation Queries

### Create Continents

```python
continents = client.query("createContinent", continent_data)
continent_ids = {continent['continent'][0]['name']:continent['continent'][0]['id'] for continent in continents}
```

### Create Countries

```python
for country in europe:
    country['continent_id'] = continent_ids['Europe']
europe_ids = {country['country'][0]['name']:country['country'][0]['id'] for country in client.query('createCountry', europe)}

for country in north_america:
    country['continent_id'] = continent_ids['North America']
north_america_ids = {country['country'][0]['name']:country['country'][0]['id'] for country in client.query('createCountry', north_america)}

for country in asia:
    country['continent_id'] = continent_ids['Asia']
asia_ids = {country['country'][0]['name']:country['country'][0]['id'] for country in client.query('createCountry', asia)}

for country in south_america:
    country['continent_id'] = continent_ids['South America']
south_america_ids = {country['country'][0]['name']:country['country'][0]['id'] for country in client.query('createCountry', south_america)}
```

### Create Cities and Set Capitals

```python
for city in uk:
    city['country_id'] = europe_ids['UK']
uk_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', uk)}
client.query('setCapital', {'country_id': europe_ids['UK'], 'city_id': uk_ids['London']})

for city in germany:
    city['country_id'] = europe_ids['Germany']
germany_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', germany)}
client.query('setCapital', {'country_id': europe_ids['Germany'], 'city_id': germany_ids['Berlin']})

for city in italy:
    city['country_id'] = europe_ids['Italy']
italy_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', italy)}
client.query('setCapital', {'country_id': europe_ids['Italy'], 'city_id': italy_ids['Rome']})

for city in china:
    city['country_id'] = asia_ids['China']
china_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', china)}
client.query('setCapital', {'country_id': asia_ids['China'], 'city_id': china_ids['Beijing']})

for city in japan:
    city['country_id'] = asia_ids['Japan']
japan_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', japan)}
client.query('setCapital', {'country_id': asia_ids['Japan'], 'city_id': japan_ids['Tokyo']})

for city in usa:
    city['country_id'] = north_america_ids['USA']
usa_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', usa)}
client.query('setCapital', {'country_id': north_america_ids['USA'], 'city_id': usa_ids['Washington, D.C.']})

for city in mexico:
    city['country_id'] = north_america_ids['Mexico']
mexico_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', mexico)}
client.query('setCapital', {'country_id': north_america_ids['Mexico'], 'city_id': mexico_ids['Mexico City']})

for city in brazil:
    city['country_id'] = south_america_ids['Brazil']
brazil_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', brazil)}
client.query('setCapital', {'country_id': south_america_ids['Brazil'], 'city_id': brazil_ids['Brasília']})

for city in argentina:
    city['country_id'] = south_america_ids['Argentina']
argentina_ids = {city['city'][0]['name']:city['city'][0]['id'] for city in client.query('createCity', argentina)}
client.query('setCapital', {'country_id': south_america_ids['Argentina'], 'city_id': argentina_ids['Buenos Aires']})
```

## Calling Read Queries

### Get All Continents

```python
print("Continents: ", [continent['name'] for continent in client.query('getAllContinents', {})[0]['continents']], "\n")
```

### Get All Countries in Europe

```python
print("Countries in Europe: ", [country['name'] for country in client.query('getAllCountries', {'continent_id': continent_ids['Europe']})[0]['countries']], "\n")
```

### Get All Cities in Japan

```python
print("Cities in Japan: ", [city['name'] for city in client.query('getAllCities', {'country_id': asia_ids['Japan']})[0]['cities']], "\n")
```

### Get Capital Cities

```python
# Get capital of Japan
print("Capital of Japan: ", client.query('getCapital', {'country_id': asia_ids['Japan']})[0]['capital'][0]['name'], "\n")

# Get capital of USA
print("Capital of USA: ", client.query('getCapital', {'country_id': north_america_ids['USA']})[0]['capital'][0]['name'], "\n")

# Get capital of Argentina
print("Capital of Argentina: ", client.query('getCapital', {'country_id': south_america_ids['Argentina']})[0]['capital'][0]['name'], "\n")
```

## Deleting the Instance

```bash
helix delete <instance-id>
```

You can find your instance ID by running `helix instances`.

## Next Steps

You've learned how to create, deploy, and delete your own instance, as well as how to write and run queries using the Python SDK.

Going from local testing to production? [Helix Cloud](../../helix-cloud/overview) makes it effortless. We handle servers, scaling, and maintenance so you can focus on building your application.

Ready to dive deeper? Check out our [guides and tutorials](../../guides/overview) for real-world use cases and advanced scenarios.

Get to know [HelixQL](../hql/hql), our fast, efficient query language built for traversing and manipulating graph and vector data.

Build, query, and embed entirely in your language of choice using our [Python SDK](../sdks/helix-py).

Discover everything HelixDB has to offer with our cutting-edge [features](../../features/overview).

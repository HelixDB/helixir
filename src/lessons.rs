use std::vec;

#[derive(Debug)]
pub struct Lesson {
    pub id: usize,
    pub title: String,
    pub instructions: String,
    pub hints: Vec<String>,
    #[allow(dead_code)]
    pub query_name: Option<Vec<String>>,
}

pub fn get_lesson(lesson_id: usize) -> Lesson {
    match lesson_id {
        0 => Lesson {
            id: 0,
            title: "Setup - Initialize HelixDB".into(),
            instructions: "Run 'helix init' to set up your helix instance (you can run it straight in this CLI)\nAlso you should open up helix docs to help you figure out best ways to write queries\nIf you get stuck along the way you can refer to the answers provided in lesson_answers folder.".into(),
            hints: vec!["Check if helixdb-cfg folder exists".into()],
            query_name: None,   
        },
        1 => Lesson {
            id: 1,
            title: "Schema Design - Nodes".into(),
            instructions: "Great, now you might have noticed that a helixdb-cfg folder appeared. In there you will start writing the schema for this tutorial.\nYou will model the relationships between continents, countries, and cities as a graph.\n\nFirst, you have to define what kind of entities/nodes will be in your graph. You will start with 3 types of nodes: continents, countries, and cities.\n\nNode Definitions:\n- The Continent node will have a name property (String)\n- The Country node will have: name (String), currency (String), population (U64), and gdp (F64)\n- The City node will have: name (String), description (String), and zip_codes (array of strings)\n\nCreate a Continent, Country, and City node with their respective properties in schema.hx, then run c to check your answer.".into(),
            hints: vec!["Use N:: for nodes".into()],
            query_name: None,
        },
        2 => Lesson {
            id: 2,
            title: "Adding in Edges".into(),
            instructions: "Now that you know what type of nodes are in your schema, you will define the relationships between those nodes.\n\nFor this example, there is a hierarchical pattern:\n- A **of city** is in a **country**\n- A **country** is in a **continent**\n\nCreate a Continent_to_Country and Country_to_City edge connecting their respective nodes with no properties in **schema.hx**".into(),
            hints: vec!["Use E:: for edges".into()],
            query_name: None,
        },
        3 => Lesson {
            id: 3,
            title: "Meta Relationships".into(),
            instructions: "In addition to the structural relationships between the nodes, you can also define relationships based on metadata. For example, a country must have a capital city.\n\nCreate a Country_to_Capital edge connecting Country to City in **schema.hx**".into(),
            hints: vec!["Use E:: for edges".into()],
            query_name: None,
        },
        4 => Lesson {
            id: 4,
            title: "Defining Vectors".into(),
            instructions: "Vectors in HelixDB allow you to create vector-based searches for semantic similarity.\n\nA **vector** is an array of floating-point numbers that represents the semantic meaning of data. In this case, you'll create a vector for city descriptions to enable semantic search capabilities.\n\nCreate a CityDescription vector with vector property that takes an array of F64 ([F64])".into(),
            hints: vec!["Use V:: for vectors".into()],
            query_name: None,
        },
        5 => Lesson {
            id: 5,
            title: "Basic Node Creation".into(),
            instructions: "Now that we have our schema, we need to write queries to insert the data. The best way to go about this given the structure of our data is to go from **top (broad)** to **bottom (narrow)** of the hierarchy.\n\nFirst, we will start with a basic query called createContinent to create a continent.\n\n**Key Points:**\n- Creation queries almost always include all the properties of the node in the arguments\n- In this case, we only need to know the continent's name\n- Use **AddN** to add a **Continent** node with property **name**\n\n**Query Parameters:** name: String\n\n**Important:** Don't forget to run **helix deploy** to deploy your schema and queries.".into(),
            hints: vec!["Add this header into your query.hx: QUERY createContinent (name: String) =>".into()],
            query_name: Some(vec!["createContinent".into()]),
        },
        6 => Lesson {
            id: 6,
            title: "Basic Node Creation".into(),
            instructions: "Most of the nodes in our schema are related to other nodes, which means that we have to also create edges between them. However, we can optimize this process by creating both the node and the edge connecting it to other existing nodes in one query.\n\n In this exercise, you will create a country node and connect it to its corresponding continent node. First create a new **Country** node using **AddN**. Then you will get the **Continent** node via the node's ID so that we can create a **Continent_to_Country** edge going from the created **Continent** to **Country** node\nusing **AddE**.We will also do the same thing for creating a city node. The query names should be createCountry and createCity\n\n**Query Parameters:**\n- createCountry: continent_id: ID, name: String, currency: String, population: I64, gdp: F64\n- createCity: country_id: ID, name: String, description: String".into(),
            hints: vec!["Add this header into your query.hx: QUERY createCity (country_id: ID, name: String, description: String) =>".into(),
            "Add this header into your query.hx: QUERY createCountry (continent_id: ID, name: String, currency: String, population: I64, gdp: F64) =>".into()],
            query_name: Some(vec!["createCountry".into(), "createCity".into()]),
        },
        7 => Lesson {
            id: 7,
            title: "Creating Meta Relationships".into(),
            instructions: "In order to add meta relationships into our graph, you will connect nodes together with the edges that define the meta relationships.For this example, you will create a Country_to_Capital edge from a Country node to a City node.\n\nWrite a query (setCapital) to set a City node as the capital city of a Country node using their IDs.\n\n**Query Parameters:** country_id: ID, city_id: ID".into(),
            hints: vec!["Add this header into your query.hx: QUERY setCapital (country_id: ID, city_id: ID) =>".into()],
            query_name: Some(vec!["setCapital".into()]),
        },
        8 => Lesson {
            id: 8,
            title: "Creating Vector Embeddings".into(),
            instructions: "Vector embeddings allow us to perform similarity-based searches on our data. For city descriptions, this means we can find cities with similar characteristics even if they don't share exact properties. We will create a vector embedding for each city's description.\nWrite a query (embedDescription) to create a CityDescription vector and connect it to its respective City node by city ID.\n\n**Query Parameters:** city_id: ID, vector: [F64]".into(),
            hints: vec!["Add this header into your query.hx: QUERY embedDescription (city_id: ID, vector: [F64]) =>".into()],
            query_name: Some(vec!["embedDescription".into()]),
        },
        9 => Lesson {
            id: 9,
            title: "Get Nodes by ID".into(),
            instructions: "Now that we know how to create nodes and their relationships, we need to be able to retrieve nodes from our graph. The simplest way is to retrieve nodes when we know their ID. Write 3 queries to get Continent (getContinent), Country (getCountry), and City (getCity) by node ID.\n\n**Query Parameters:**\n- getContinent: continent_id: ID\n- getCountry: country_id: ID\n- getCity: city_id: ID".into(),
            hints: vec!["Add this header into your query.hx: QUERY getContinent (continent_id: ID) =>".into()],
            query_name: Some(vec!["getContinent".into(),"getCountry".into(), "getCity".into()]),
        },
        10 => Lesson {
            id: 10,
            title: "Get All Nodes of Type".into(),
            instructions: "In addition to retrieving nodes by ID, we often want to retrieve all nodes of a certain type. Since we have a hierarchical structure, we will also want to get all countries within a continent and all cities within a country.\n\nWrite 3 queries to get all **Continent** (getAllContinents), **Country** (getAllCountries), **City** nodes (getAllCities), 2 queries to get all **Country** (getCountriesInContinent) and **City** (getCitiesInCountry) nodes by their parent IDs (you can use **Out** for this after getting parent nodes from their ID\n\n**Query Parameters:**\n- getAllContinents: (no parameters)\n- getAllCountries: (no parameters)\n- getAllCities: (no parameters)\n- getCountriesInContinent: continent_id: ID\n- getCitiesInCountry: country_id: ID".into(),
            hints: vec!["Add this header into your query.hx: QUERY getAllContinents () =>".into()],
            query_name: Some(vec!["getAllContinents".into(),"getAllCountries".into(), "getAllCities".into(),"getCountriesInContinent".into(), "getCitiesInCountry".into()]),
        },
        11 => Lesson {
            id: 11,
            title: "Get Nodes by Meta Relationship".into(),
            instructions: "Similar to getting nodes by their hierarchical relationships, we can also get nodes via their meta relationships. For this example, we will retrieve the capital city of a country. We'll do this by traversing the **Country_to_Capital** edge from a **Country** node to find its capital **City** node.\n\nWrite a query **getCapital** to get a country's capital **City** node by the country's ID.\n\n**Query Parameters:** country_id: ID".into(),
            hints: vec!["Add this header into your query.hx: QUERY getCapital (country_id: ID) =>".into()],
            query_name: Some(vec!["getCapital".into()]),
        },
        12 => Lesson {
            id: 12,
            title: "Get Node Properties".into(),
            instructions: "Sometimes you don't need the full node, just a few specific properties. For example, you can display only the names and populations of countries without pulling in the entire object. In this case, you can use property selection syntax to retrieve just the fields you care about. This allows for more efficient querying and cleaner data handling when building visualizations or summaries.\n\nWrite a query (**getCountryNames**) to get each country's **name** and **population**\n\n**Query Parameters:** (no parameters)".into(),
            hints: vec!["Add this header into your query.hx: QUERY getCountryNames () =>".into(), "Use property selection syntax ::={name, population}".into()],
            query_name: Some(vec!["getCountryNames".into()]),
        },
        13 => Lesson {
            id: 13,
            title: "Get Nodes by Property".into(),
            instructions: "In addition to retrieving nodes by their ID or relationship, you often need to find nodes based on their properties. This allows for more flexible querying of your graph database. You'll write queries to retrieve nodes by specific properties they contain.\n\nWrite 3 queries that get the **Continent** (**getContinentByName**), **Country** (**getCountryByName**), and **City** (**getCityByName**) nodes by their names.\n\n**Query Parameters:**\n- getContinentByName: continent_name: String\n- getCountryByName: country_name: String\n- getCityByName: city_name: String".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getContinentByName (continent_name: String) =>".into(), 
                "Use WHERE clause with property matching: ::WHERE(_::{name}::EQ(continent_name))".into()
            ],
            query_name: Some(vec!["getContinentByName".into(), "getCountryByName".into(), "getCityByName".into()]),
        },
        14 => Lesson {
            id: 14,
            title: "Filtering with WHERE Conditions".into(),
            instructions: "Building on property-based queries, you can also filter nodes using comparison operators. This allows you to find nodes that meet specific criteria rather than exact matches. You'll practice with different comparison operators to filter countries by various attributes.\n\nWrite 3 queries to filter countries: one by currency (**getCountriesByCurrency**) (exact match), one by population (**getCountriesByPopulation**) (less than a value), and one by GDP (**getCountriesByGdp**) (greater than or equal to a value).\n\n**Query Parameters:**\n- getCountriesByCurrency: currency: String\n- getCountriesByPopulation: max_population: I64\n- getCountriesByGdp: min_gdp: F64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getCountriesByCurrency (currency: String) =>".into(),
                "Add this header into your query.hx: QUERY getCountriesByPopulation (max_population: I64) =>".into(),
                "Add this header into your query.hx: QUERY getCountriesByGdp (min_gdp: F64) =>".into(),
                "Use comparison operators: ::EQ() for equality, ::LT() for less than, ::GTE() for greater than or equal".into()
            ],
            query_name: Some(vec!["getCountriesByCurrency".into(), "getCountriesByPopulation".into(), "getCountriesByGdp".into()]),
        },
        15 => Lesson {
            id: 15,
            title: "Get Nodes by Many Properties".into(),
            instructions: "Now that you've seen how to get nodes by individual properties, you can also combine multiple conditions to perform more advanced filtering. For this example, you'll write queries that retrieve Country nodes based on a combination of property values. This includes filtering countries with a population greater than a minimum and a GDP less than or equal to a maximum, as well as retrieving countries that either use a specific currency or have a population below a certain threshold. These types of queries allow you to refine your searches and extract more targeted subsets of data from your graph.\n\nWrite a query (**getCountriesByPopGdp**) to find **Country** nodes with both population greater than **min_population** and GDP less than or equal to **max_gdp**.\n\nWrite a query (**getCountriesByCurrPop**) to find **Country** nodes with either a specific **currency** or a population less than or equal to **max_population**.\n\n**Query Parameters:**\n- getCountriesByPopGdp: min_population: I64, max_gdp: F64\n- getCountriesByCurrPop: currency: String, max_population: I64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getCountriesByPopGdp (min_population: I64, max_gdp: F64) =>".into(),
                "Add this header into your query.hx: QUERY getCountriesByCurrPop (currency: String, max_population: I64) =>".into(),
                "Use AND() for combining conditions with logical AND: AND(_::{population}::GT(min_population), _::{gdp}::LTE(max_gdp))".into(),
                "Use OR() for combining conditions with logical OR: OR(_::{currency}::EQ(currency), _::{population}::LTE(max_population))".into()
            ],
            query_name: Some(vec!["getCountriesByPopGdp".into(), "getCountriesByCurrPop".into()]),
        },
        16 => Lesson {
            id: 16,
            title: "Get Nodes by Meta Relationships".into(),
            instructions: "In addition to traversing structural relationships, you can also query nodes based on meta relationships. For example, you can retrieve all **Country** nodes that have a capital city assigned. This involves checking for the existence of an outgoing **Country_to_Capital** edge from each **Country** node. Meta relationship queries like this are useful for identifying nodes with specific contextual connections beyond hierarchical structures.\n\nWrite a query (**getCountriesWithCapitals**) to get **Country** nodes that have capital cities.\n\n**Query Parameters:** (no parameters)".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getCountriesWithCapitals () =>".into(),
                "Use EXISTS() to check for the existence of an outgoing edge: WHERE(EXISTS(_::Out<Country_to_Capital>))".into()
            ],
            query_name: Some(vec!["getCountriesWithCapitals".into()]),
        },
        17 => Lesson {
            id: 17,
            title: "Get Range of Nodes".into(),
            instructions: "When working with large datasets, it's often useful to limit the number of results returned from a query. The RANGE operator allows you to implement pagination and control result set size efficiently. This is particularly important for performance when dealing with queries that might return many nodes. The RANGE operator takes two parameters: the starting index (0-based) and the number of items to return.\n\nWrite a query (**getContinentCities**) to get the first k (I64) City nodes in a continent given the continent's name.\n\n**Query Parameters:** continent_name: String, k: I64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getContinentCities (continent_name: String, k: I64) =>".into(),
                "Use RANGE(0, k) to limit results: ::Out<Country_to_City>::RANGE(0, k)".into(),
                "Chain the traversals: continent -> countries -> cities with RANGE applied to the final result".into()
            ],
            query_name: Some(vec!["getContinentCities".into()]),
        },
        18 => Lesson {
            id: 18,
            title: "Get Count of Nodes".into(),
            instructions: "In some cases, you want to gather basic statistics about your graph. For example, you can count the number of capital cities by checking how many City nodes have an incoming Country_to_Capital edge. Using the COUNT operation, you can quickly compute aggregate statistics like this to better understand the structure and distribution of data across your graph.\n\nWrite a query (**countCapitals**) to get the number of capital cities.\n\n**Query Parameters:** (no parameters)".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY countCapitals () =>".into(),
                "Use WHERE with EXISTS to find cities that are capitals: WHERE(EXISTS(_::In<Country_to_Capital>))".into(),
                "Use COUNT operation to count the matching nodes: ::COUNT".into()
            ],
            query_name: Some(vec!["countCapitals".into()]),
        },
        19 => Lesson {
            id: 19,
            title: "Get Nodes with Anonymous Traversals".into(),
            instructions: "Sometimes you want to filter nodes based on other node's properties. For example, you can get all countries that have more than a certain number of cities. To do this, you'll count the number of outgoing Country_to_City edges from each Country node and filter by num_cities. This pattern of anonymous traversal is useful when you care about the structure or degree of connectivity in the graph, rather than the specific linked nodes themselves.\n\nWrite a query (**getCountryByCityCnt**) to get Country nodes that has more cities than num_cities.\n\n**Query Parameters:** num_cities: I64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY getCountryByCityCnt (num_cities: I64) =>".into(),
                "Use WHERE with anonymous traversal: WHERE(_::Out<Country_to_City>::COUNT()>$(num_cities))".into(),
                "Anonymous traversal syntax: _:: means we don't care about the target nodes, just the count".into()
            ],
            query_name: Some(vec!["getCountryByCityCnt".into()]),
        },
        20 => Lesson {
            id: 20,
            title: "Semantic Search Vectors".into(),
            instructions: "Semantic search allows you to go beyond exact matches by comparing the meaning of data. For example, you can find cities with similar descriptions using vector embeddings. By searching against CityDescription vectors, you can retrieve the top-k most semantically similar City nodes to a given input vector. This is especially useful when you want to find cities that share common characteristics or themes, even if their properties don't match exactly.\n\nFor this lesson, you're using fake embeddings to test the semantic search functionality. In a real application, you would use proper embeddings from models like OpenAI or other embedding providers.\n\nWrite a query (**searchDescriptions**) to semantically search a vector against CityDescription vectors and returning the top k City nodes.\n\n**Query Parameters:** vector: [F64], k: I64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY searchDescriptions (vector: [F64], k: I64) =>".into(),
                "Use SearchV<CityDescription>(vector, k) to perform semantic search".into(),
                "Connect search results to City nodes using traversal: descriptions<-CityDescription to Embedding->city".into()
            ],
            query_name: Some(vec!["searchDescriptions".into()]),
        },
        21 => Lesson {
            id: 21,
            title: "Updating Nodes".into(),
            instructions: "Updating nodes allows you to modify the properties of existing entities in your graph without needing to recreate them. To update a node, you use the UPDATE operation followed by the fields you want to modify. For example, you can update a country's currency by its ID, or simultaneously update both its population and GDP. Keeping node data up-to-date ensures your graph remains accurate and relevant for queries, visualizations, and downstream analytics.\n\nWrite a query (**updateCurrency**) to update a country's currency by a country's ID.\n\nWrite a query (**updatePopGdp**) to update a country's population and gdp by a country's ID.\n\n**Query Parameters:**\n- updateCurrency: country_id: ID, currency: String\n- updatePopGdp: country_id: ID, population: I64, gdp: F64".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY updateCurrency (country_id: ID, currency: String) =>".into(),
                "Add this header into your query.hx: QUERY updatePopGdp (country_id: ID, population: I64, gdp: F64) =>".into(),
                "Use UPDATE operation: country <- N<Country>(country_id)::UPDATE({currency: currency})".into(),
                "For multiple fields: country <- N<Country>(country_id)::UPDATE({population: population, gdp: gdp})".into()
            ],
            query_name: Some(vec!["updateCurrency".into(), "updatePopGdp".into()]),
        },
        22 => Lesson {
            id: 22,
            title: "Updating Meta Relationships".into(),
            instructions: "Sometimes you need to update the meta relationships between nodes rather than creating new ones. For example, you might want to change which city serves as a country's capital. This involves removing the existing capital relationship and creating a new one with a different city. When updating meta relationships, it's important to properly manage the edge connections to maintain graph consistency.\n\nWrite a query (**updateCapital**) to update the capital City node of a Country node given the country's ID and the new capital city's ID.\n\n**Query Parameters:** country_id: ID, city_id: ID".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY updateCapital (country_id: ID, city_id: ID) =>".into(),
                "First DROP the existing capital edge: DROP N<Country>(country_id)::OutE<Country_to_Capital>".into(),
                "Then get the country and city nodes to create new relationship".into(),
                "Use AddE<Country_to_Capital> to create the new capital relationship".into()
            ],
            query_name: Some(vec!["updateCapital".into()]),
        },
        23 => Lesson {
            id: 23,
            title: "Updating Embeddings".into(),
            instructions: "When working with vector embeddings, you often need to update both the node properties and their associated vector embeddings. For example, when a city's description changes, you need to update the description property and also update the corresponding vector embedding to reflect the new semantic meaning. This ensures that semantic searches remain accurate and relevant.\n\nWrite a query (**updateDescription**) to update the description of a City node given its ID and also update the CityDescription vector embedding given a new vector.\n\n**Query Parameters:** city_id: ID, description: String, vector: [F64]".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY updateDescription (city_id: ID, description: String, vector: [F64]) =>".into(),
                "First DROP the existing embedding: DROP N<City>(city_id)::OutE<City_to_Embedding>".into(),
                "Update the city description using UPDATE operation".into(),
                "Add new vector embedding with AddV<CityDescription>(vector)".into()
            ],
            query_name: Some(vec!["updateDescription".into()]),
        },
        24 => Lesson {
            id: 24,
            title: "Deleting Nodes".into(),
            instructions: "Deleting nodes is useful when you want to clean up outdated or incorrect data from your graph. However, this can get tricky in a graph database because not only do you have to drop the node but also the relationships connected to that node. Additionally, the order in which you drop them is very important. For example, if a city is no longer relevant or a country needs to be removed entirely, you can drop the node and its relationship to the country as a city and also potentially as a capital city. In cases where the node is linked through specific edges, like a capital city connection, it's important to remove those edges first to maintain the graph structure and allowing you to drop the other edges later. This ensures that dependent edges don't linger in the system, avoiding potential inconsistencies during traversal or analytics.\n\nWrite a query (**deleteCity**) to delete a City node given its ID.\n\nWrite a query (**deleteCapital**) to delete a capital City node given its country's ID.\n\nWrite a query (**deleteCountry**) to delete a Country node given its ID.\n\n**Query Parameters:**\n- deleteCity: city_id: ID\n- deleteCapital: country_id: ID\n- deleteCountry: country_id: ID".into(),
            hints: vec![
                "Add this header into your query.hx: QUERY deleteCity (city_id: ID) =>".into(),
                "Add this header into your query.hx: QUERY deleteCapital (country_id: ID) =>".into(),
                "Add this header into your query.hx: QUERY deleteCountry (country_id: ID) =>".into(),
                "Use DROP operation: DROP N<City>(city_id)".into(),
                "For capital deletion: DROP N<Country>(country_id)::Out<Country_to_Capital>".into(),
                "For country deletion: DROP N<Country>(country_id)".into()
            ],
            query_name: Some(vec!["deleteCity".into(), "deleteCapital".into(), "deleteCountry".into()]),
        },
        _ => Lesson {
            id: lesson_id,
            title: "Lesson Not Found".into(),
            instructions: "This lesson hasn't been implemented yet.".into(),
            hints: vec!["Try going back to a previous lesson.".into()],
            query_name: None
        }
    }
}

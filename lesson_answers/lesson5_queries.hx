QUERY createContinent (name: String) =>
    continent <- AddN<Continent>({name: name})
    RETURN continent
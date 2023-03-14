$version: "2.0"

namespace mcnamara.nz

use aws.protocols#restJson1

/// The Solar System Catalog is a retrieval system for information about
/// planets and other bodies in the solar system.
@title("SolarSystemCatalog")
@restJson1
service SolarSystemCatalog {
    version: "2023-03-01",
    operations: [
        GetPlanet,
    ]
}

/// Retrieve information about a Pokémon species.
@readonly
@http(uri: "/planet/{name}", method: "GET")
operation GetPlanet {
    input: GetPlanetInput,
    output: GetPlanetOutput,
    errors: [NotFound],
}

/// Input parameters
@input
structure GetPlanetInput {
    /// The name for the planet to search.
    @required
    @httpLabel
    name: String,
}

@output
structure GetPlanetOutput {
    /// The name for this resource.
    @required
    name: String,

    @required
    descriptions: Descriptions,
}

list Descriptions {
    member: Description
}

structure Description {
    /// The localized content
    @required
    content: String,

    @required
    language: Language,
}

/// Supported languages for Description entries.
@enum([
    {
        name: "ENGLISH",
        value: "en",
        documentation: "American English.",
    },
    {
        name: "SPANISH",
        value: "es",
        documentation: "Español.",
    },
    {
        name: "ITALIAN",
        value: "it",
        documentation: "Italiano.",
    },
    {
        name: "JAPANESE",
        value: "jp",
        documentation: "日本語。",
    },
])
string Language

// Return 404 to the client if the requested Pokémon does not exist.
@error("client")
@httpError(404)
structure NotFound {
    @required
    message: String,
}

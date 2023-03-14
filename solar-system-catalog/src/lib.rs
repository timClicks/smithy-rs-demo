//! Solar system catalog
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
use std::sync::Arc;

use aws_smithy_http_server::Extension;
use solar_system_catalog_server_sdk::{error, input, output};
use tracing_subscriber::{prelude::*, EnvFilter};

mod state;
pub mod tls;

pub use state::State;

/// Setup `tracing::subscriber` to read the log level from RUST_LOG environment variable.
pub fn setup_tracing() {
    let format = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_line_number(true)
        .with_level(true);
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(format)
        .with(filter)
        .init();
}

/// Retrieves information about a planet in the catalog
pub async fn get_planet(
    input: input::GetPlanetInput,
    state: Extension<Arc<State>>,
) -> Result<output::GetPlanetOutput, error::GetPlanetError> {
    let name = &input.name;

    tracing::debug!("Requested planet: {name}");

    if let Some(translations) = state.0.descriptions.get(name) {
        tracing::info!("Found planet in catalog: {name}");
        let output = output::GetPlanetOutput {
            name: input.name,
            descriptions: translations.to_vec(),
        };
        Ok(output)

    } else {
        tracing::info!("Not found: {name}");
        Err(error::NotFound {
            message: String::from("Requested solar system body not found."),
        }
        .into())
    }
}

//! PokémonService shared state.
use std::collections::HashMap;

use solar_system_catalog_server_sdk::model::{Description, Language};

/// Some applications may want to manage state between handlers. Imagine having a database connection pool
/// that can be shared between different handlers and operation implementations.
/// State management can be expressed in a struct where the attributes hold the shared entities.
///
/// **NOTE: It is up to the implementation of the state structure to handle concurrency by protecting**
/// **its attributes using synchronization mechanisms.**
///
/// The framework stores the `Arc<T>` inside an [`http::Extensions`] and conveniently passes it to
/// the operation's implementation, making it able to handle operations with two different async signatures:
/// * `FnOnce(InputType) -> Future<OutputType>`
/// * `FnOnce(InputType, Extension<Arc<T>>) -> Future<OutputType>`
///
/// Wrapping the service with a [`tower::Layer`] will allow to have operations' signatures with and without shared state:
///
/// ```compile_fail
/// use std::sync::Arc;
/// use aws_smithy_http_server::{AddExtensionLayer, Extension, Router};
/// use tower::ServiceBuilder;
/// use tokio::sync::RwLock;
///
/// // Shared state,
/// #[derive(Debug, State)]
/// pub struct State {
///     pub count: RwLock<u64>
/// }
///
/// // Operation implementation with shared state.
/// async fn operation_with_state(input: Input, state: Extension<Arc<State>>) -> Output {
///     let mut count = state.0.write().await;
///     *count += 1;
///     Ok(Output::new())
/// }
///
/// // Operation implementation without shared state.
/// async fn operation_without_state(input: Input) -> Output {
///     Ok(Output::new())
/// }
///
/// let app: Router = OperationRegistryBuilder::default()
///     .operation_with_state(operation_with_state)
///     .operation_without_state(operation_without_state)
///     .build()
///     .unwrap()
///     .into();
/// let shared_state = Arc::new(State::default());
/// let app = app.layer(ServiceBuilder::new().layer(AddExtensionLayer::new(shared_state)));
/// let server = hyper::Server::bind(&"0.0.0.0:13734".parse().unwrap()).serve(app.into_make_service());
/// ...
/// ```
///
/// Without the middleware layer, the framework will require operations' signatures without
/// the shared state.
///
/// [`middleware`]: [`aws_smithy_http_server::AddExtensionLayer`]
#[derive(Debug)]
pub struct State {
    pub(crate) descriptions: HashMap<String, Vec<Description>>,
}

impl Default for State {
    fn default() -> Self {
        let mut descriptions = HashMap::new();

        descriptions.insert(
            String::from("Earth"),
            vec![
                Description {
                    content: "Earth is the third planet from the Sun and the only place known in the universe where life has originated and found habitability.".to_string(),
                    language: Language::English,
                },
                Description {
                    content: "La Terra è il terzo pianeta in ordine di distanza dal Sole e il più grande dei pianeti terrestri del sistema solare, sia per massa sia per diametro.".to_string(),
                    language: Language::Italian,
                },
                Description {
                    content: "La Tierra (del latín Terra,17​ deidad romana equivalente a Gea, diosa griega de la feminidad y la fecundidad) es un planeta del sistema solar que gira alrededor de su estrella —el Sol— en la tercera órbita más interna. ".to_string(),
                    language: Language::Spanish,
                },
                Description {
                    content: "地球（ちきゅう、羅: Terra、英: The Earth）は、太陽系の惑星の1つ[5]。太陽から3番目に近いため、太陽系第３惑星と言われる。表面に水、空気中に酸素を大量に蓄え、人類を含む多種多様な生命体が生存することを特徴とする惑星である[6]。".to_string(),
                    language: Language::Japanese,
                },
            ]
        );
        Self {
            descriptions,
        }
    }
}

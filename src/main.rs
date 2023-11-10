pub mod assets;
mod entsoe;
mod error;
pub mod models;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use entsoe::EntsoeClient;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod forecast {
    use crate::{
        assets::{DocumentType, ProcessType, PsrType, AREA_CODE},
        AppState,
    };
    use axum::extract::{Query, State};

    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Params {
        pub area_code: AREA_CODE,
        pub process_type: ProcessType,
        pub document_type: DocumentType,
        pub psr_type: PsrType,
    }

    pub async fn forecast(params: Query<Params>, State(state): State<AppState>) -> String {
        let params: Params = params.0;
        let client = state.entsoe_client;
        let result = client
            .with_area_code(params.area_code)
            .with_process_type(params.process_type)
            .with_document_type(params.document_type)
            .with_psr_type(params.psr_type)
            .request()
            .await;
        result.unwrap()
    }
}

#[derive(Clone)]
pub struct AppState {
    entsoe_client: EntsoeClient,
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            hello::world,
        ),
        components(
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
                )
            }
        }
    }

    let entsoe_api_key =
        std::env::var("ENTSOE_API_KEY").expect("ENTSOE_API_KEY is undefined env var");
    // build our application with a single route
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/forecast", get(forecast::forecast))
        .with_state(AppState {
            entsoe_client: EntsoeClient::new(entsoe_api_key),
        });

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod hello {
    use axum::Json;

    /// List all Todo items
    ///
    /// List all Todo items from in-memory storage.
    #[utoipa::path(
        get,
        path = "/world",
        responses(
            (status = 200, description = "List all todos successfully", body = [Todo])
        )
    )]
    pub(super) async fn world() -> Json<Vec<String>> {
        let return_string = "Hello, World!".to_owned();
        Json(vec![return_string])
    }
}

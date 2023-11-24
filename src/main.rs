pub mod assets;
mod entsoe;
mod error;

pub mod models;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use entsoe::EntsoeClient;
use tracing::{event, info, span, Level};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use models::*;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub mod forecast {
    use crate::{
        assets::{DocumentType, ProcessType, PsrType, AREA_CODE},
        AppState,
    };
    use axum::extract::{Query, State};
    use axum::Json;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    type DateType = DateTime<chrono::Utc>;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Params {
        pub period_start: DateType,
        pub period_end: DateType,
        pub in_domain: AREA_CODE,
        pub process_type: ProcessType,
        pub document_type: DocumentType,
        pub psr_type: PsrType,
    }

    #[utoipa::path(
        get,
        path = "/forecast",
        responses(
            (status = 200, description = "Get forecast", body = GLMarketDocument)
        )
    )]
    pub async fn forecast(params: Query<Params>, State(state): State<AppState>) -> Json<String> {
        let params: Params = params.0;
        let client = state.entsoe_client;
        let result = client
            .with_period_start(params.period_start)
            .with_period_end(params.period_end)
            .with_in_domain(params.in_domain)
            .with_process_type(params.process_type) // day ahead
            .with_document_type(params.document_type)
            .with_psr_type(params.psr_type)
            .request()
            .await;
        let as_json = result.map(|x| serde_json::to_string(&x).unwrap()).unwrap();
        println!("sending bytes {:?}", as_json.len());
        Json(as_json)
    }
}

#[derive(Clone)]
pub struct AppState {
    entsoe_client: EntsoeClient,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder();
    let subscriber = subscriber.with_max_level(tracing::Level::INFO);
    tracing::subscriber::set_global_default(subscriber.finish())
        .expect("setting tracing default failed");

    info!("starting server");
    dotenv().unwrap();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            forecast::forecast,
        ),
        components(
            schemas(GLMarketDocument, TimePeriodTimeInterval, TimeSeries, Period, Point, TimeInterval, ReceiverMarketParticipantMRID, SenderMarketParticipantMRID, MktPSRType, InBiddingZoneDomainMRID)
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
